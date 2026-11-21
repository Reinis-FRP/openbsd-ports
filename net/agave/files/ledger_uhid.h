#ifndef LEDGER_UHID_H
#define LEDGER_UHID_H

#include <sys/types.h>

__BEGIN_DECLS
int	agave_ledger_product(int, uint16_t, char *, char *, size_t);
int	agave_ledger_wait(int, int);
__END_DECLS

#endif
