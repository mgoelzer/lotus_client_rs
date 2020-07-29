//use serde_cbor::value::Value;

// Example:  this base64 should deserialize to the structure below
//
// gtgqWBkAAVUAFGZpbC8xL3BheW1lbnRjaGFubmVsWEqCVQHh5IHxtn/1yPh3sahOqrXKYJiP2Fgx
// A5COeZC7Z3Ejrc/ZS2SrqwUBCOL6J79wYaYKeynU4PM8jrXQrNErYVA3XkXy0+p9lQ==
//
// 82                                      # array(2)
//    D8 2A                                # tag(42)
//       58 19                             # bytes(25)
//          000155001466696C2F312F7061796D656E746368616E6E656C // ID CID "fil/1/paymentchannel"
//    58 4A                                # bytes(74)
//          825501E1E481F1B67FF5C8F877B1A84EAAB5CA60988FD8583103908E7990BB677123ADCFD94B64ABAB050108E2FA27BF7061A60A7B29D4E0F33C8EB5D0ACD12B6150375E45F2D3EA7D95
//
// the latter 74 bytes encoding:
//
// 82                                      # array(2)
//    55                                   # bytes(21)
//       01E1E481F1B67FF5C8F877B1A84EAAB5CA60988FD8  // (t1)4hsid4nwp724r6dxwgue5kvvzjqjrd6y(4lalecq)
//    58 31                                # bytes(49)
//       03908E7990BB677123ADCFD94B64ABAB050108E2FA27BF7061A60A7B29D4E0F33C8EB5D0ACD12B6150375E45F2D3EA7D95
//                                                   // (t3)schhtef3m5yshlop3ffwjk5lauaqryx2e67xayngbj5stvha6m6i5noqvtiswykqg5pel4wt5j6zl(rbblxbq)

//#[derive(Debug)]

// TODO: write me!