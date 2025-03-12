use std::collections::HashMap;

#[derive(Eq, Hash, PartialEq, Clone, Debug)]
pub struct ItemObject {
    item_name: String,
    item_price: u8,
}

pub struct ItemPrices {
    item_prices_map: HashMap<String, u8>,
}

pub struct ComboOffers {
    combo_price: u8,
    offers: Vec<(ItemObject, ItemObject)>,
}

impl ItemObject {
    pub fn new(item_name: String, item_price: u8) -> Self {
        Self { item_name, item_price }
    }

    pub fn get_price(&self) -> u8 {
        self.item_price
    }

    pub fn get_name(&self) -> &str {
        &self.item_name
    }
}

impl ItemPrices {
    pub fn new() -> Self {
        Self {
            item_prices_map: HashMap::new(),
        }
    }

    pub fn add_item(&mut self, item: ItemObject) {
        if self.item_prices_map.insert(item.get_name().to_string(), item.get_price()).is_some() {
            println!("{} Item already exists in the Item Prices",item.get_name());
        }
    }

    pub fn remove_item(&mut self, item_name: &str) {
        if self.item_prices_map.remove(item_name).is_none() {
            println!("{} Item does not exist in the Item Prices",item_name);
        }
    }

    pub fn get_item(&self, item_name: &str) -> Option<ItemObject> {
        self.item_prices_map.get(item_name)
            .map(|&price| ItemObject::new(item_name.to_string(), price))
            .or_else(|| {
                println!("{} Item does not exist in the Item Prices",item_name);
                None
            })
    }

    pub fn update_price(&mut self, item_name: &str, item_price: u8) {
        if let Some(price) = self.item_prices_map.get_mut(item_name) {
            *price = item_price;
        } else {
            println!("{} Item does not exist in the Item Prices",item_name);
        }
    }
}

impl ComboOffers {
    pub fn new() -> Self {
        Self {
            offers: Vec::new(),
            combo_price: 0,
        }
    }

    pub fn add_offer(&mut self, item1: ItemObject, item2: ItemObject) {
        self.offers.push((item1, item2));
    }

    pub fn set_combo_price(&mut self, combo_price: u8, item_prices: &ItemPrices) {
        self.combo_price = combo_price;
        self.create_offers(item_prices);
    }

    pub fn get_combo_price(&self) -> u8 {
        self.combo_price
    }

    pub fn create_offers(&mut self, item_prices_map: &ItemPrices) {
        let mut price_to_items: HashMap<u8, Vec<ItemObject>> = HashMap::new();

        for (name, &price) in &item_prices_map.item_prices_map {
            price_to_items.entry(price)
                .or_insert_with(Vec::new)
                .push(ItemObject::new(name.clone(), price));
        }

        for (&price, items) in &price_to_items {
            if let Some(remaining_price) = self.combo_price.checked_sub(price) {
                if let Some(other_items) = price_to_items.get(&remaining_price) {
                    for item1 in items {
                        for item2 in other_items {
                            if item1 != item2 {
                                self.add_offer(item1.clone(), item2.clone());
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn generate_offers(&self) -> Option<Vec<(ItemObject, ItemObject)>> {
        if self.combo_price == 0 || self.offers.is_empty() {
            println!("No offers created, check offer price");
            None
        } else {
            Some(self.offers.clone())
        }
    }
}