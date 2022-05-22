use super::{Screen, ScreenFrame};
use crate::api::proto::TradeInfo;
use tui::{
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem},
};

pub struct TradesScreen {
    trades: Vec<TradeInfo>,
}

impl TradesScreen {
    pub fn new(trades: Vec<TradeInfo>) -> Self {
        Self { trades }
    }
}

impl Screen for TradesScreen {
    fn paint(&self, frame: &mut ScreenFrame) {
        let items = self
            .trades
            .iter()
            .map(|t| ListItem::new(t.trade_id.clone()))
            .collect::<Vec<ListItem>>();
        let list = List::new(items)
            .block(Block::default().title("Trades").borders(Borders::ALL))
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
            .highlight_symbol(">>");
        let size = frame.size();

        frame.render_widget(list, size);
    }
}
