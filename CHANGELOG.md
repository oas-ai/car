# Changelog

## Unreleased

- Add the read-only Genesis G80 2017 adapter and decoded-frame integration test.

## [0.1.0] - 2026-09-21

- Canonical Vehicle Model용 Rust crate 초기 구조와 CI를 추가했습니다.
- `sdk` VehicleState contract와 정렬된 read-only 상태 모델을 추가했습니다.
- `oas-can` decode contract를 사용하는 Manufacturer Adapter 경계와 state freshness 검사를 추가했습니다.
- Synthetic CAN frame부터 VehicleState까지의 end-to-end contract test를 추가했습니다.
