# VehicleState Mapping

`car`의 `VehicleState`는 `sdk`의 `oas.vehicle.v1.VehicleState`와 같은 canonical 의미와 SI 단위를 사용합니다.

- 관측되지 않은 scalar는 `Option::None`으로 유지합니다. `0`, `false`와 혼동하지 않습니다.
- 제조사별 CAN signal 이름, scale, offset은 Adapter 내부에서만 사용합니다.
- Adapter는 DBC parser의 결과를 이 모델로 변환하고, 상위 Application에는 OEM별 값을 노출하지 않습니다.
- 이 모델은 read-only 상태입니다. 제어 명령은 별도 API와 Safety 경계를 통해 설계합니다.
