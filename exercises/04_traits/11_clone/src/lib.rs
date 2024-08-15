// TODO: add the necessary `Clone` implementations (and invocations)
//  to get the code to compile.

pub fn summary(ticket: Ticket) -> (Ticket, Summary) {
    let ticket_clone: Ticket = ticket.clone();
    let _summary: Summary = ticket_clone.clone().summary();
    (ticket_clone, _summary)
}

#[derive(Clone)]
pub struct Ticket {
    pub title: String,
    pub description: String,
    pub status: String,
}

impl Ticket {
    pub fn summary(self) -> Summary {
        Summary {
            title: self.title,
            status: self.status,
        }
    }
}

pub struct Summary {
    pub title: String,
    pub status: String,
}
