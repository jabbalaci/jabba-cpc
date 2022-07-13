use jabba_cpc;

#[test]
fn set_text_and_get_text_test() {
    let backup = jabba_cpc::clipboard::get_text();
    //
    let text = "";
    jabba_cpc::clipboard::set_text(text);
    assert_eq!(jabba_cpc::clipboard::get_text(), text);

    let text = "a";
    jabba_cpc::clipboard::set_text(text);
    assert_eq!(jabba_cpc::clipboard::get_text(), text);

    let text = "hello";
    jabba_cpc::clipboard::set_text(text);
    assert_eq!(jabba_cpc::clipboard::get_text(), text);
    //
    jabba_cpc::clipboard::set_text(&backup);
    assert_eq!(jabba_cpc::clipboard::get_text(), backup);
}
