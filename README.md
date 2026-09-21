# car

OAS Canonical Vehicle Model, 제조사 Adapter 및 Vehicle Control의 Safety 경계를 담당하는 Rust library입니다.

OEM signal은 이 저장소의 Adapter 경계에서만 처리하며 상위 API에는 노출하지 않습니다.

`GenesisG80LegacyAdapter`는 승인된 Genesis G80 2017 DBC의 `CLU11`, `SAS11`, `TCS13`
read-only 신호를 SI 단위의 `VehicleState`로 변환합니다.
