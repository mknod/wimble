use wimble::command_parser::{parse_command, CommandAction};
use wimble::browser::BrowserCommand;
use tokio::sync::mpsc;

#[tokio::test]
async fn test_parse_command_up() {
    let (tx, _rx) = mpsc::channel(1);
    let action = parse_command("!up", "!", &tx).await;
    if let CommandAction::SendToBrowser(BrowserCommand::PredefinedKey(key)) = action {
        assert_eq!(format!("{:?}", key), "Up");
    } else {
        panic!("Unexpected command action");
    }
}

#[tokio::test]
async fn test_parse_command_toggle_element() {
    let (tx, _rx) = mpsc::channel(1);
    let action = parse_command("!toggle_submit", "!", &tx).await;
    if let CommandAction::SendToBrowser(BrowserCommand::ClickElement(ref s)) = action {
        assert_eq!(s, "submit");
    } else {
        panic!("Expected ClickElement");
    }
}

#[tokio::test]
async fn test_parse_command_non_command() {
    let (tx, _) = mpsc::channel(1);
    let action = parse_command("hello world", "!", &tx).await;
    matches!(action, CommandAction::Noop);
}
