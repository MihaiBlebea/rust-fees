use chrono::{DateTime, Utc, Duration};
use uuid::Uuid;
use std::fmt::{Error, Display, Formatter, Result as Res};
use std::result::{Result};

fn main() {
    let chip_lite = Plan::new(0, Currency::GBP, Duration::days(28));
    let chip_ai = Plan::new(150, Currency::GBP, Duration::days(28));
    let chip_x = Plan::new(300, Currency::GBP, Duration::days(28));

    let mut sub_chip_ai = Subscription::new(chip_ai);

    let three_days_ago = past_date(3);
    sub_chip_ai.update_start_date(three_days_ago);

    let res = calculate_price_diff(sub_chip_ai, chip_x);
    println!("{:?}", res);
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Currency {
    USD = 1,
    EUR = 2,
    RON = 3,
    GBP = 4
}

impl Currency {
    fn eq(&self, currency: &Currency) -> bool {
        self == currency
    }
}

impl Display for Currency {
    fn fmt(&self, f: &mut Formatter<'_>) -> Res {
        write!(f, "({})", self)
    }
}

struct Money {
    amount: u32,
    currency: Currency
}

impl Money {
    fn new(amount: u32, currency:Currency) -> Self {
        Money{amount, currency}
    }

    fn eq(&self, money: &Money) -> bool {
        self.amount == money.amount
    }

    fn gt(&self, money: &Money) -> bool {
        self.amount > money.amount
    }

    fn lt(&self, money: &Money) -> bool {
        self.amount < money.amount
    }

    fn sub(&self, money: &Money) -> Result<Money, Error> {
        if self.currency.eq(&money.currency) == false {
            Err(Error{})
        } else {
            let money = Money::new(&self.amount - money.amount, self.currency);

            Ok(money)
        }
    }
}

#[derive(Debug)]
struct Plan {
    id: String,
    fee: u32,
    currency: Currency,
    transition: String,
    billing_cycle: Duration
}

impl Plan {
    fn new(fee: u32, currency: Currency, billing_cycle: Duration) -> Self {
        let id = Uuid::new_v4().to_string().clone();
        let transition = id.clone();
        Plan{id, fee, currency, transition, billing_cycle}
    }

    fn set_transition(mut self, transition: String) {
        self.transition = transition
    }
}

impl Print for Plan {
    fn print(&self) {
        println!("{}, {}, {}, {}", self.id, self.fee, self.currency, self.transition)
    }
}

struct Subscription {
    id: String,
    plan: Plan,
    start_at: DateTime<Utc>,
    end_at: DateTime<Utc>
}

impl Subscription {
    fn new(plan: Plan) -> Self {
        let id = Uuid::new_v4().to_string();
        let start_at: DateTime<Utc> = Utc::now();
        let end_at = start_at +  Duration::days(28);

        Subscription{id, plan, start_at, end_at}
    }

    fn update_start_date(&mut self, start_at: DateTime<Utc>) {
        self.start_at = start_at
    }

    fn update_end_date(mut self, end_at: DateTime<Utc>) {
        self.end_at = end_at
    }
}

trait Print {
    fn print(&self);
}

fn calculate_price_diff(subscription: Subscription, next_plan: Plan) -> u32 {
    if subscription.plan.fee > next_plan.fee {
        return 0 as u32;
    }

    let now: DateTime<Utc> = Utc::now();
    let days = (now - subscription.start_at).num_days();

    let fee = subscription.plan.fee;
    let fee_per_day = fee / (subscription.plan.billing_cycle.num_days() as u32);
    let current_fee = fee_per_day * (days as u32);

    next_plan.fee - current_fee
}

fn past_date(days: i64) -> DateTime<Utc> {
    let now: DateTime<Utc> = Utc::now();
    now -  Duration::days(days)
}


#[test]
fn test_calculate_price_diff_upgrade() {
    let chip_ai = Plan::new(150, Currency::GBP, Duration::days(28));
    let chip_x = Plan::new(300, Currency::GBP, Duration::days(28));

    let mut sub_chip_ai = Subscription::new(chip_ai);

    let three_days_ago = past_date(3);
    sub_chip_ai.update_start_date(three_days_ago);

    let res = calculate_price_diff(sub_chip_ai, chip_x);

    assert_eq!(res, 285)
}

#[test]
fn test_calculate_price_diff_downgrade() {
    let chip_lite = Plan::new(0, Currency::GBP, Duration::days(28));
    let chip_ai = Plan::new(150, Currency::GBP, Duration::days(28));

    let sub_chip_ai = Subscription::new(chip_ai);

    let res = calculate_price_diff(sub_chip_ai, chip_lite);

    assert_eq!(res, 0)
}