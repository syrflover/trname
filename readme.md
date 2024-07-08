# trname

[The TV DB](https://thetvdb.com) 시즌 네이밍에 맞게 파일과 폴더 이름을 변경해줌

## 주의
- Special(Movie, OVA)은 지원하지 않음

## 설치

```bash
git clone https://github.com/syrflover/trname.git
cargo install --path ./trname
```

## 사용

1. 제목 `Lycoris Recoil` 폴더를 생성
2. `[Moozzi2] Lycoris Recoil [ x265-10Bit Ver. ] - TV + SP` 를 제목 폴더 안으로 이동함
3. trname 실행
```bash
trname "/Lycoris Recoil/[Moozzi2] Lycoris Recoil [ x265-10Bit Ver. ] - TV + SP"
```
4. 시즌 넘버 수정
```
/Lycoris Recoil/[Moozzi2] Lycoris Recoil [ x265-10Bit Ver. ] - TV + SP

Lycoris Recoil

> season: 1
```
5. 변경될 파일 이름 및 폴더 이름 확인
```
"[Moozzi2] Lycoris Recoil - 01 (BD 1920x1080 x265-10Bit FLACx2).mkv" -> "Lycoris Recoil S01E01.mkv"
"[Moozzi2] Lycoris Recoil - 01 (BD 1920x1080 x265-10Bit FLACx2).smi" -> "Lycoris Recoil S01E01.smi"
"[Moozzi2] Lycoris Recoil - 02 (BD 1920x1080 x265-10Bit FLACx2).mkv" -> "Lycoris Recoil S01E02.mkv"
"[Moozzi2] Lycoris Recoil - 02 (BD 1920x1080 x265-10Bit FLACx2).smi" -> "Lycoris Recoil S01E02.smi"
"[Moozzi2] Lycoris Recoil - 03 (BD 1920x1080 x265-10Bit Flac).mkv" -> "Lycoris Recoil S01E03.mkv"
"[Moozzi2] Lycoris Recoil - 03 (BD 1920x1080 x265-10Bit Flac).smi" -> "Lycoris Recoil S01E03.smi"
"[Moozzi2] Lycoris Recoil - 04 (BD 1920x1080 x265-10Bit Flac).mkv" -> "Lycoris Recoil S01E04.mkv"
"[Moozzi2] Lycoris Recoil - 04 (BD 1920x1080 x265-10Bit Flac).smi" -> "Lycoris Recoil S01E04.smi"
"[Moozzi2] Lycoris Recoil - 05 (BD 1920x1080 x265-10Bit Flac).mkv" -> "Lycoris Recoil S01E05.mkv"
"[Moozzi2] Lycoris Recoil - 05 (BD 1920x1080 x265-10Bit Flac).smi" -> "Lycoris Recoil S01E05.smi"
"[Moozzi2] Lycoris Recoil - 06 (BD 1920x1080 x265-10Bit Flac).mkv" -> "Lycoris Recoil S01E06.mkv"
"[Moozzi2] Lycoris Recoil - 06 (BD 1920x1080 x265-10Bit Flac).smi" -> "Lycoris Recoil S01E06.smi"
"[Moozzi2] Lycoris Recoil - 07 (BD 1920x1080 x265-10Bit Flac).mkv" -> "Lycoris Recoil S01E07.mkv"
"[Moozzi2] Lycoris Recoil - 07 (BD 1920x1080 x265-10Bit Flac).smi" -> "Lycoris Recoil S01E07.smi"
"[Moozzi2] Lycoris Recoil - 08 (BD 1920x1080 x265-10Bit Flac).mkv" -> "Lycoris Recoil S01E08.mkv"
"[Moozzi2] Lycoris Recoil - 08 (BD 1920x1080 x265-10Bit Flac).smi" -> "Lycoris Recoil S01E08.smi"
"[Moozzi2] Lycoris Recoil - 09 (BD 1920x1080 x265-10Bit Flac).mkv" -> "Lycoris Recoil S01E09.mkv"
"[Moozzi2] Lycoris Recoil - 09 (BD 1920x1080 x265-10Bit Flac).smi" -> "Lycoris Recoil S01E09.smi"
"[Moozzi2] Lycoris Recoil - 10 (BD 1920x1080 x265-10Bit Flac).mkv" -> "Lycoris Recoil S01E10.mkv"
"[Moozzi2] Lycoris Recoil - 10 (BD 1920x1080 x265-10Bit Flac).smi" -> "Lycoris Recoil S01E10.smi"
"[Moozzi2] Lycoris Recoil - 11 (BD 1920x1080 x265-10Bit Flac).mkv" -> "Lycoris Recoil S01E11.mkv"
"[Moozzi2] Lycoris Recoil - 11 (BD 1920x1080 x265-10Bit Flac).smi" -> "Lycoris Recoil S01E11.smi"
"[Moozzi2] Lycoris Recoil - 12 (BD 1920x1080 x265-10Bit FLACx2).mkv" -> "Lycoris Recoil S01E12.mkv"
"[Moozzi2] Lycoris Recoil - 12 (BD 1920x1080 x265-10Bit FLACx2).smi" -> "Lycoris Recoil S01E12.smi"
"[Moozzi2] Lycoris Recoil - 13 END (BD 1920x1080 x265-10Bit FLACx2).mkv" -> "Lycoris Recoil S01E13.mkv"
"[Moozzi2] Lycoris Recoil - 13 END (BD 1920x1080 x265-10Bit FLACx2).smi" -> "Lycoris Recoil S01E13.smi"
"[Moozzi2] Lycoris Recoil [ x265-10Bit Ver. ] - TV + SP" -> "Season 01"

title: Lycoris Recoil
season: 1

> confirm [y/n]:
```
6. 최종적으로는 아래와 같은 폴더 구조가 됨
```
/Lycoris Recoil
└── Season 01
    ├── Lycoris Recoil S01E01.mkv
    ├── Lycoris Recoil S01E01.smi
    ├── Lycoris Recoil S01E02.mkv
    ├── Lycoris Recoil S01E02.smi
    ├── Lycoris Recoil S01E03.mkv
    ├── Lycoris Recoil S01E03.smi
    ├── Lycoris Recoil S01E04.mkv
    ├── Lycoris Recoil S01E04.smi
    ├── Lycoris Recoil S01E05.mkv
    ├── Lycoris Recoil S01E05.smi
    ├── Lycoris Recoil S01E06.mkv
    ├── Lycoris Recoil S01E06.smi
    ├── Lycoris Recoil S01E07.mkv
    ├── Lycoris Recoil S01E07.smi
    ├── Lycoris Recoil S01E08.mkv
    ├── Lycoris Recoil S01E08.smi
    ├── Lycoris Recoil S01E09.mkv
    ├── Lycoris Recoil S01E09.smi
    ├── Lycoris Recoil S01E10.mkv
    ├── Lycoris Recoil S01E10.smi
    ├── Lycoris Recoil S01E11.mkv
    ├── Lycoris Recoil S01E11.smi
    ├── Lycoris Recoil S01E12.mkv
    ├── Lycoris Recoil S01E12.smi
    ├── Lycoris Recoil S01E13.mkv
    └── Lycoris Recoil S01E13.smi
```
