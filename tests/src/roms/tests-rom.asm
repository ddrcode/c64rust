; Super minimalistic ROM that just sets the jump vectors
; The ROM is created for a sole purpose of integration tests.
; Reset vector is set to $0200, as all tests are expected
; to start from that address.

* = $fffa
nmi !word $0000
rst !word $0200
irq !word $0000


