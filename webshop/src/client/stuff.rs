use crate::client::api::get_orders;
use crate::client::webmodels::OrderRequest;
use crate::db::orderitems::{order_items_insert, DbNewOrderItem};
use crate::db::orders::{order_insert, order_last_order_created, DbNewOrder};
use crate::error::webshoperror::WebshopError;
use chrono::{TimeDelta, Utc};
use std::ops::{Add, Sub};

pub async fn find_and_insert_orders(
    pool: &deadpool_diesel::postgres::Pool,
) -> Result<usize, WebshopError> {
    let last_order = order_last_order_created(pool).await?;
    println!("last_order: {:?}", last_order);

    let tmp = Utc::now().sub(TimeDelta::days(365 * 50));

    let mut last_order_date = match last_order {
        Some(last) => last.order_created.and_utc(),
        None => tmp,
    };

    let page_size = 25;
    let mut total_orders_inserted = 0;
    let mut total_items_inserted = 0;
    loop {
        let order_request = OrderRequest {
            last_order_created: last_order_date,
            page_size,
        };

        println!("order_request: {:?}", order_request);

        let new_orders = get_orders(order_request).await?;

        if new_orders.is_empty() {
            println!("no orders found in response. breaking out of loop");
            break;
        }

        let db_new_orders: Vec<DbNewOrder> = new_orders
            .iter()
            .map(|o| DbNewOrder {
                order_id: o.order_id.clone(),
                buyer_id: o.buyer_id.clone(),
                buyer_name: Some(o.buyer_name.clone()),
                erp_order_number: o.erp_order_number.clone(),
                state: o.state.clone(),
                additional_info_1: o.additional_info_1.clone(),
                additional_info_2: o.additional_info_2.clone(),
                number_items: o.number_items,
                order_created: o.order_created.naive_utc(),
            })
            .collect();

        let order_cnt = db_new_orders.len();

        total_orders_inserted += order_cnt;
        println!(
            "order_cnt {}, total_orders_inserted: {}",
            order_cnt, total_orders_inserted
        );

        let r = order_insert(pool, db_new_orders).await?;
        println!("last inserted order id: {:?}", r.order_id);

        for new_order in new_orders {
            let new_items: Vec<DbNewOrderItem> = new_order
                .items
                .iter()
                .map(|item| DbNewOrderItem {
                    order_id: new_order.order_id.clone(),
                    item_id: item.item_id.clone(),
                    name: Some(item.name.clone()),
                    description: Some(item.description.clone()),
                    price: item.price,
                    state: item.state.clone(),
                    additional_info_1: item.additional_info_1.clone(),
                    additional_info_2: item.additional_info_2.clone(),
                    item_created: item.item_created.naive_utc(),
                })
                .collect();
            let items_cnt = new_items.len();
            total_items_inserted += items_cnt;

            let _ = order_items_insert(pool, new_items).await?;
        }

        if order_cnt < page_size as usize {
            println!("{} orders in response, that's less than the page_size of {} -> breaking out of loop ", order_cnt, page_size);
            break;
        }

        last_order_date = order_last_order_created(pool)
            .await?
            .expect("should be an order")
            .order_created
            .and_utc()
            // why on earth is this  working on Mac OSX without this, but not on ubuntu linux 24.10 ??
            .add(TimeDelta::milliseconds(1));

        println!("last_order_created: {:?}", last_order_date);
    }
    println!("finished inserting {} orders.", total_orders_inserted);
    Ok(total_orders_inserted)
}
