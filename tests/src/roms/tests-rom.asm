; Super minimalistic ROM that just sets the jump vectors
; The ROM is created for a sole purpose of integration tests.
; Reset vector is set to $0200, as all tests are expected
; to start from that address.

* = $fff0
    RTI

* = $fffa
nmi !word $fff0
rst !word $0200
irq !word $fff0

