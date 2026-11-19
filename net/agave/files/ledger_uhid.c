/* Probe the same open file that will carry the Ledger reports. */
#include <sys/types.h>
#include <sys/ioctl.h>
#include <dev/usb/usb.h>
#include <usbhid.h>
#include <string.h>

int
agave_ledger_product(int fd, uint16_t vendor, char *model, char *serial,
    size_t size)
{
	struct usb_device_info info;
	report_desc_t desc;
	hid_data_t parser;
	hid_item_t item;
	int id, product = -1;

	if (ioctl(fd, USB_GET_DEVICEINFO, &info) == -1 ||
	    info.udi_vendorNo != vendor ||
	    ioctl(fd, USB_GET_REPORT_ID, &id) == -1 || id != 0)
		return -1;
	if ((desc = hid_get_report_desc(fd)) == NULL)
		return -1;
	if (hid_report_size(desc, hid_input, id) != 64 ||
	    hid_report_size(desc, hid_output, id) != 64)
		goto out;
	if ((parser = hid_start_parse(desc, 1 << hid_collection, id)) == NULL)
		goto out;
	while (hid_get_item(parser, &item) > 0) {
		/* Ledger application collection, distinct from FIDO. */
		if (item.kind == hid_collection && item.collection == 1 &&
		    item.usage == 0xffa00001) {
			strlcpy(model, info.udi_product, size);
			strlcpy(serial, info.udi_serial, size);
			product = info.udi_productNo;
			break;
		}
	}
	hid_end_parse(parser);
out:
	hid_dispose_report_desc(desc);
	return product;
}
