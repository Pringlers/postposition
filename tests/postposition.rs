use postposition::josa;

#[test]
fn test_ko() {
    assert_eq!(josa("철수", "이", "가"), "가"); // 철수가
    assert_eq!(josa("홍길동", "이", "가"), "이"); // 홍길동이
    assert_eq!(josa("100점", "을", "를"), "을"); // 100점을
    assert_eq!(josa("너", "과", "와"), "와"); // 너와
    assert_eq!(josa("사랑", "과", "와"), "과"); // 사랑과

    assert_eq!(josa("뿗", "은", "는"), "은"); // 뿗은
    assert_eq!(josa("뀙", "과", "와"), "과"); // 뀙과

    assert_eq!(josa("ㄹ", "은", "는"), "은"); // 리을은
    assert_eq!(josa("ㅟ", "을", "를"), "를"); // 위를

    assert_eq!(josa("코로나바이러스감염증-19", "이", "가"), "가"); // [...] 십구가
    assert_eq!(josa("아이폰 11", "은", "는"), "은"); // [...] 십일은
}

#[test]
fn test_num() {
    assert_eq!(josa("1", "을", "를"), "을"); // 일을
    assert_eq!(josa("2", "은", "는"), "는"); // 이는
    assert_eq!(josa("3", "과", "와"), "과"); // 삼과
}

#[test]
fn test_en() {
    assert_eq!(josa("Yuna", "이", "가"), "가"); // 유나가
    assert_eq!(josa("Rust", "을", "를"), "를"); // 러스트를
    assert_eq!(josa("Juliet", "과", "와"), "과"); // 줄리엣과

    assert_eq!(josa("Rust 1.6", "은", "는"), "는"); // [...] 원 포인트 식스는
    assert_eq!(josa("WWDC23", "이", "가"), "가"); // [...] 투웬티 쓰리가

    assert_eq!(josa("p", "은", "는"), "는"); // 피는
    assert_eq!(josa("r", "이", "가"), "이"); // 알이
}

#[test]
fn test_others() {
    assert_eq!(josa("こんにちは", "을", "를"), "");
    assert_eq!(josa("", "이", "가"), "");
}
