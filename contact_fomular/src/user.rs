use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct UserData {
    id: Uuid,
    name: String,
    age: u8,
    location: Location,
}

#[derive(Debug, Clone)]
pub struct Location {
    street: String,
    city: String,
    postal_code: u16,
    country: String,
}
#[derive(Debug)]
pub enum DataSelector<'a> {
    Id(Uuid),
    Name(&'a str),
    Age(u8),
    Street(&'a str),
    PostalCode(u16),
    City(&'a str),
    Country(&'a str),
}

impl UserData {
    pub fn new() -> Self {
        UserData {
            id: Uuid::new_v4(),
            name: String::new(),
            age: 0,
            location: Location {
                street: String::new(),
                city: String::new(),
                postal_code: 0,
                country: String::new(),
            },
        }
    }
    pub fn get_data_id(&self) -> Uuid {
        self.id
    }
    pub fn get_data_name(&self) -> &str {
        &self.name.as_str()
    }
    pub fn get_data_age(&self) -> &u8 {
        &self.age
    }
    pub fn get_data_street(&self) -> &str {
        &self.location.street.as_str()
    }
    pub fn get_data_postal_code(&self) -> &u16 {
        &self.location.postal_code
    }
    pub fn get_data_city(&self) -> &str {
        &mut self.location.city.as_str()
    }
    pub fn get_data_country(&self) -> &str {
        &mut self.location.country.as_str()
    }

    pub fn set_data(&mut self, data: DataSelector) {
        match data {
            DataSelector::Id(new_id) => {
                self.id = new_id;
            }
            DataSelector::Name(new_name) => {
                self.name = new_name.to_string();
            }
            DataSelector::Age(new_age) => {
                self.age = new_age;
            }
            DataSelector::Street(new_street) => {
                self.location.street = new_street.to_string();
            }
            DataSelector::PostalCode(new_postal_code) => {
                self.location.postal_code = new_postal_code;
            }
            DataSelector::City(new_city) => {
                self.location.city = new_city.to_string();
            }
            DataSelector::Country(new_country) => {
                self.location.country = new_country.to_string();
            }
        }
    }
}
