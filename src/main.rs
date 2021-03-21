use chrono::{DateTime, Utc, Duration};
use uuid::Uuid;
use std::fmt::{Error, Display, Formatter, Result as Res};
use std::result::{Result};

fn main() {
    let chip_lite = Plan::new(Money::new(0, Currency::GBP), Duration::days(28));
    let chip_ai = Plan::new(Money::new(150, Currency::GBP), Duration::days(28));
    let chip_x = Plan::new(Money::new(300, Currency::GBP), Duration::days(28));

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

#[derive(Debug)]
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
    money: Money,
    transition: String,
    billing_cycle: Duration
}

impl Plan {
    fn new(money: Money, billing_cycle: Duration) -> Self {
        let id = Uuid::new_v4().to_string().clone();
        let transition = id.clone();
        Plan{id, money, transition, billing_cycle}
    }

    fn set_transition(mut self, transition: String) {
        self.transition = transition
    }
}

impl Print for Plan {
    fn print(&self) {
        println!("id {} - cost {}{} - transition to {}", self.id, self.money.amount, self.money.currency, self.transition)
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

fn calculate_price_diff(subscription: Subscription, next_plan: Plan) -> Money {
    if subscription.plan.money.gt(&next_plan.money) {
        return Money::new(0, Currency::GBP);
    }

    let now: DateTime<Utc> = Utc::now();
    let days = (now - subscription.start_at).num_days();

    let fee_per_day = subscription.plan.money.amount / (subscription.plan.billing_cycle.num_days() as u32);
    let current_fee = Money::new(fee_per_day * (days as u32), Currency::GBP);

    next_plan.money.sub(&current_fee).expect("Currency should be the same")
}

fn past_date(days: i64) -> DateTime<Utc> {
    let now: DateTime<Utc> = Utc::now();
    now -  Duration::days(days)
}


#[test]
fn test_calculate_price_diff_upgrade() {
    let chip_ai = Plan::new(Money::new(150, Currency::GBP), Duration::days(28));
    let chip_x = Plan::new(Money::new(300, Currency::GBP), Duration::days(28));

    let mut sub_chip_ai = Subscription::new(chip_ai);

    let three_days_ago = past_date(3);
    sub_chip_ai.update_start_date(three_days_ago);

    let res = calculate_price_diff(sub_chip_ai, chip_x);

    assert_eq!(res.amount, 285);
    assert_eq!(res.currency, Currency::GBP);
}

#[test]
fn test_calculate_price_diff_downgrade() {
    let chip_lite = Plan::new(Money::new(0, Currency::GBP), Duration::days(28));
    let chip_ai = Plan::new(Money::new(150, Currency::GBP), Duration::days(28));

    let sub_chip_ai = Subscription::new(chip_ai);

    let res = calculate_price_diff(sub_chip_ai, chip_lite);

    assert_eq!(res.amount, 0);
    assert_eq!(res.currency, Currency::GBP);
}