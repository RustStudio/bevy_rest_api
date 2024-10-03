// ============================================================================
/*
 * Copyright (C) 2024 Rust Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 *
*/
// ============================================================================
use bevy::prelude::*;

pub struct RestPlugin;

impl Plugin for RestPlugin {
    fn build (&self, app: &mut App){
        app
        .init_resource::<StoreResource>()
        .add_plugins(bevy_tokio_tasks::TokioTasksPlugin::default())
        .add_systems(PreStartup, start_rest_server)
        .add_systems(FixedUpdate, publish_store_resource)
        .insert_resource(Time::<Fixed>::from_hz(1.0));
    }
}

// ============================================================================
use bevy_tokio_tasks::TokioTasksRuntime;
use warp::Filter;
use serde::{Deserialize, Serialize};

#[derive(Resource, Default, Clone)]
struct StoreResource(Store);

fn start_rest_server(
    runtime: ResMut<TokioTasksRuntime>,
    mut store_resource: ResMut<StoreResource>,
){
    let store_resource_clone = store_resource.clone();
    runtime.spawn_background_task(|mut ctx| async move {
        let store_filter = warp::any().map(move || store_resource_clone.clone());
    
        let add_items = warp::post()
            .and(warp::path("v1"))
            .and(warp::path("groceries"))
            .and(warp::path::end())
            .and(json_body())
            .and(store_filter.clone())
            .and_then(update_grocery_list);
    
        let get_items = warp::get()
            .and(warp::path("v1"))
            .and(warp::path("groceries"))
            .and(warp::path::end())
            .and(store_filter.clone())
            .and_then(get_grocery_list);
    
        let routes = add_items.or(get_items);
    
        warp::serve(routes)
            .run(([127, 0, 0, 1], 3030))
            .await;
    });

}

use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::Arc;

type Items = HashMap<String, i32>;

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
struct Item {
    name: String,
    quantity: i32,
}

#[derive(Clone, Default)]
struct Store {
  grocery_list: Arc<RwLock<Items>>
}

impl Store {
    fn new() -> Self {
        Store {
            grocery_list: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

async fn get_grocery_list(
    store: StoreResource
    ) -> Result<impl warp::Reply, warp::Rejection> {
        let result = store.0.grocery_list.read();
        eprintln!("Getting items {:?}", result);
        Ok(warp::reply::json(&*result))
}

use warp::{http};
fn json_body() -> impl Filter<Extract = (Item,), Error = warp::Rejection> + Clone {
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

async fn update_grocery_list(
    item: Item,
    store: StoreResource
    ) -> Result<impl warp::Reply, warp::Rejection> {
        store.0.grocery_list.write().insert(item.name, item.quantity);


        Ok(warp::reply::with_status(
            "Added items to the grocery list",
            http::StatusCode::CREATED,
        ))
}

fn publish_store_resource(
    store: Res<StoreResource>,
){
    let items = store.0.grocery_list.read();
    eprintln!("Publishing items {:?}", items);
}