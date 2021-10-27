use nyxx_in::*;

fn main() {
    let s = Pos::new(1, 10);
    println!("start: [{}] or [{:?}]", s, s);

    let e = Pos::new(1, 20);
    println!("end: [{}] or [{:?}]", e, e);

    let e1 = Pos::new(2, 20);
    println!("end: [{}] or [{:?}]", e1, e1);

    let span = Span::with_pos(s, e);
    println!("span: [{}] or [{:?}]", span, span);

    let span1 = Span::with_pos(s, e1);
    println!("span: [{}] or [{:?}]", span1, span1);

    let tkn = Token::new(TokenValue::And, span);
    println!("tkn [{}] or [{:?}]", tkn, tkn);
}
