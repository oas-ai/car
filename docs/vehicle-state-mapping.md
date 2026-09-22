# VehicleState Mapping

`car`의 `VehicleState`는 `sdk`의 `oas.vehicle.v1.VehicleState`와 같은 canonical 의미와 SI 단위를 사용합니다.

- 관측되지 않은 scalar는 `Option::None`으로 유지합니다. `0`, `false`와 혼동하지 않습니다.
- 제조사별 CAN signal 이름, scale, offset은 Adapter 내부에서만 사용합니다.
- Adapter는 DBC parser의 결과를 이 모델로 변환하고, 상위 Application에는 OEM별 값을 노출하지 않습니다.
- 이 모델은 read-only 상태입니다. 제어 명령은 별도 API와 Safety 경계를 통해 설계합니다.

`ManufacturerAdapter`는 `oas-can`의 `DecodedCanMessage`만 입력으로 받아 상태를 갱신합니다. CAN transport·DBC file I/O·Raw CAN TX를 소유하지 않습니다. `VehicleState::is_fresh_at`은 timestamp가 없거나 future/stale인 상태를 consumer가 유효한 상태로 취급하지 않게 합니다.

`tests/synthetic_pipeline.rs`는 fixture frame을 decode한 뒤 adapter가 `VehicleState`를 갱신하는 경로를 검증합니다. 이 fixture는 어떤 실제 차량 signal도 나타내지 않습니다.

# Genesis G80 2017 legacy mapping

`GenesisG80LegacyAdapter`는 `can`의 DBC subset decoder가 만든 메시지만 받는다.

| DBC signal | Canonical field | 변환 |
| --- | --- | --- |
| `CLU11.CF_Clu_Vanz` | `vehicle_speed_mps` | `CF_Clu_SPEED_UNIT`에 따라 km/h 또는 mph를 m/s로 변환 |
| `SAS11.SAS_Angle` | `steering.angle_rad` | deg → rad |
| `TCS13.ACCEL_REF_ACC` | `acceleration_mps2` | m/s² |
| `TCS13.DriverOverride` | `brake.pressed` | 값 `2`일 때 `true` |

알 수 없는 메시지·단위·신호는 상태를 변경하지 않는다. 이 adapter는 CAN 송신이나 차량
제어를 수행하지 않는다.
