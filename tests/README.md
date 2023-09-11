# Integration Tests

## Test results

Registers:

- A: number of tests successfully passed
- Y: status/error codes (see the list below)
- P: processor status of the last test operation.
  I.e. in case of test for CMP operation, the P registry will be showing
  the status after the last executed CMP

## Error codes

- `00` - No error (test passed)
- `01` - General error (reason not specified)

- `10` - General flag error (flag not specified)
- `11` - Carry flag incorrect
- `12` - Zero flag incorrect
- `13` - Interrupt flag incorrect
- `14` - Decimal mode flag incorrect
- `15` - Break flag incorrect
- `17` - Overflow flag incorrect
- `18` - Negative flag incorrect
