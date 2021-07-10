pub use super::*;
use rust_decimal_macros::dec;

pub async fn test_update_cake(db: &DbConn) {
    let seaside_bakery = bakery::ActiveModel {
        name: Set("SeaSide Bakery".to_owned()),
        profit_margin: Set(10.4),
        ..Default::default()
    };
    let bakery_insert_res: InsertResult = Bakery::insert(seaside_bakery)
        .exec(db)
        .await
        .expect("could not insert bakery");

    let mud_cake = cake::ActiveModel {
        name: Set("Mud Cake".to_owned()),
        price: Set(dec!(10.25)),
        gluten_free: Set(false),
        bakery_id: Set(Some(bakery_insert_res.last_insert_id as i32)),
        ..Default::default()
    };

    let cake_insert_res: InsertResult = Cake::insert(mud_cake)
        .exec(db)
        .await
        .expect("could not insert cake");

    let cake: Option<cake::Model> = Cake::find_by_id(cake_insert_res.last_insert_id)
        .one(db)
        .await
        .expect("could not find cake");

    assert!(cake.is_some());
    let cake_model = cake.unwrap();
    assert_eq!(cake_model.name, "Mud Cake");
    assert_eq!(cake_model.price, dec!(10.25));
    assert_eq!(cake_model.gluten_free, false);

    let mut cake_am: cake::ActiveModel = cake_model.into();
    cake_am.name = Set("Extra chocolate mud cake".to_owned());
    cake_am.price = Set(dec!(20.00));

    let _cake_update_res: cake::ActiveModel = Cake::update(cake_am)
        .exec(db)
        .await
        .expect("could not update cake");

    let cake: Option<cake::Model> = Cake::find_by_id(cake_insert_res.last_insert_id)
        .one(db)
        .await
        .expect("could not find cake");
    let cake_model = cake.unwrap();
    assert_eq!(cake_model.name, "Extra chocolate mud cake");
    assert_eq!(cake_model.price, dec!(20.00));
}

pub async fn test_update_bakery(db: &DbConn) {
    let seaside_bakery = bakery::ActiveModel {
        name: Set("SeaSide Bakery".to_owned()),
        profit_margin: Set(10.4),
        ..Default::default()
    };
    let bakery_insert_res: InsertResult = Bakery::insert(seaside_bakery)
        .exec(db)
        .await
        .expect("could not insert bakery");

    let bakery: Option<bakery::Model> = Bakery::find_by_id(bakery_insert_res.last_insert_id)
        .one(db)
        .await
        .expect("could not find bakery");

    assert!(bakery.is_some());
    let bakery_model = bakery.unwrap();
    assert_eq!(bakery_model.name, "SeaSide Bakery");
    assert_eq!(bakery_model.profit_margin, 10.4);

    let mut bakery_am: bakery::ActiveModel = bakery_model.into();
    bakery_am.name = Set("SeaBreeze Bakery".to_owned());
    bakery_am.profit_margin = Set(12.00);

    let _bakery_update_res: bakery::ActiveModel = Bakery::update(bakery_am)
        .exec(db)
        .await
        .expect("could not update bakery");

    let bakery: Option<bakery::Model> = Bakery::find_by_id(bakery_insert_res.last_insert_id)
        .one(db)
        .await
        .expect("could not find bakery");
    let bakery_model = bakery.unwrap();
    assert_eq!(bakery_model.name, "SeaBreeze Bakery");
    assert_eq!(bakery_model.profit_margin, 12.00);
}
