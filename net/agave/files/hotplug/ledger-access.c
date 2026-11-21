#include <sys/types.h>
#include <sys/stat.h>
#include <sys/wait.h>
#include <dev/usb/usb.h>
#include <err.h>
#include <errno.h>
#include <fcntl.h>
#include <grp.h>
#include <stdio.h>
#include <string.h>
#include <unistd.h>

int agave_ledger_product(int, uint16_t, char *, char *, size_t);

/* Product IDs in Agave v4.4.1 remote-wallet/src/ledger.rs. */
static int
supported_product(int product)
{
	static const struct {
		int legacy, first, last;
	} models[] = {
		{ 0x0001, 0x1000, 0x101f }, /* Nano S */
		{ 0x0004, 0x4000, 0x401f }, /* Nano X */
		{ 0x0005, 0x5000, 0x501f }, /* Nano S Plus */
		{ 0x0006, 0x6000, 0x601f }, /* Stax */
		{ 0x0007, 0x7000, 0x701f }, /* Flex */
		{ 0x0008, 0x8000, 0x801f }, /* Nano Gen5 */
	};
	size_t i;

	for (i = 0; i < sizeof(models) / sizeof(models[0]); i++) {
		if (product == models[i].legacy ||
		    (product >= models[i].first && product <= models[i].last))
			return 1;
	}
	return 0;
}

/* The parent holds the event lock until creation and probing finish. */
static void
create_node(const char *device)
{
	pid_t pid;
	int status;

	if ((pid = fork()) == -1)
		err(1, "fork");
	if (pid == 0) {
		if (chdir("/dev") == -1)
			err(1, "chdir /dev");
		execl("/bin/sh", "sh", "/dev/MAKEDEV", device, (char *)NULL);
		err(1, "exec /bin/sh");
	}
	while (waitpid(pid, &status, 0) == -1) {
		if (errno != EINTR)
			err(1, "waitpid");
	}
	if (!WIFEXITED(status) || WEXITSTATUS(status) != 0)
		errx(1, "MAKEDEV %s failed", device);
}

/* Called for both attach and detach. Reconcile the current device. */
int
main(int argc, char **argv)
{
	struct group *gr;
	struct stat st, opened;
	char path[sizeof("/dev/") + 16];
	char model[USB_MAX_STRING_LEN], serial[USB_MAX_STRING_LEN];
	const char *unit;
	gid_t group;
	int attach, fd, product, rescan;

	attach = argc == 4 && strcmp(argv[3], "--attach") == 0;
	rescan = argc == 4 && strcmp(argv[3], "--rescan") == 0;
	if (argc != 3 && !attach && !rescan)
		errx(1, "usage: ledger-access class device [--attach | --rescan]");
	if (strcmp(argv[1], "0") != 0 || strncmp(argv[2], "uhid", 4) != 0)
		return 0;
	unit = argv[2] + 4;
	if (*unit == '\0' || unit[strspn(unit, "0123456789")] != '\0' ||
	    snprintf(path, sizeof(path), "/dev/%s", argv[2]) >= (int)sizeof(path))
		return 0;
	if ((gr = getgrnam("ledgerusb")) == NULL || gr->gr_gid == 0)
		errx(1, "a dedicated ledgerusb group is required");
	group = gr->gr_gid;
	if (open("/var/run/ledger-access.lock", O_CREAT | O_RDWR | O_NOFOLLOW |
	    O_EXLOCK | O_CLOEXEC, 0600) == -1)
		err(1, "ledger-access.lock");
	if (lstat(path, &st) == -1) {
		if (errno != ENOENT)
			err(1, "%s", path);
		if (!attach)
			return 0;
		create_node(argv[2]);
		if (lstat(path, &st) == -1)
			err(1, "%s after MAKEDEV", path);
	}
	if (!S_ISCHR(st.st_mode) || st.st_uid != 0)
		return 0;
	if (st.st_gid == group) {
		/* Withdraw our previous grant even when the device is gone. */
		if (chmod(path, 0600) == -1 || chown(path, 0, 0) == -1)
			err(1, "reset %s", path);
	} else if (st.st_gid != 0 || (st.st_mode & 07777) != 0600)
		return 0;

	fd = open(path, O_RDONLY | O_NONBLOCK | O_NOFOLLOW);
	if (fd == -1) {
		/* Full scans can encounter unrelated HID nodes that return EIO. */
		if (rescan && errno == EIO && st.st_gid != group)
			return 0;
		if (errno == ENXIO || errno == EBUSY || errno == ENODEV)
			return 0;
		err(1, "open %s", path);
	}
	if (fstat(fd, &opened) == -1)
		err(1, "fstat %s", path);
	if (opened.st_dev != st.st_dev || opened.st_ino != st.st_ino ||
	    !S_ISCHR(opened.st_mode))
		return 0;
	product = agave_ledger_product(fd, 0x2c97, model, serial, sizeof(model));
	if (supported_product(product)) {
		if (fchown(fd, 0, group) == -1 || fchmod(fd, 0660) == -1)
			err(1, "grant %s", path);
	}
	return 0;
}
