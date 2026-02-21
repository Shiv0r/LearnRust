#[derive(Debug)]
pub struct UserData {
    id: String,
    name: String,
    age: u8,
    location: Location,
}

pub enum DataSelector<'a> {
    Id(&'a str),
    Name(&'a str),
    Age(u8),
    Street(&'a str),
    PostalCode(u16),
    City(&'a str),
    County(&'a str),
}

#[derive(Debug)]
pub struct Location {
    street: String,
    city: String,
    postal_code: u16,
    country: String,
}

impl UserData {
    pub fn get_data_id(&self) -> &str {
        &self.id.as_str()
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

    pub fn set_data(&mut self, data: DataSelector) {
        match data {
            DataSelector::Id(new_id) => {
                self.id = new_id.to_string();
            },
            DataSelector::Name(new_name) => {
                self.name = new_name.to_string();
            }
            DataSelector::Age(new_age) => {
                self.age = new_age;
            },
            DataSelector::Street(new_street) => {
                self.location.street = new_street.to_string();
            }
            DataSelector::PostalCode(new_postal_code) => {
                self.location.postal_code = new_postal_code;
            }
            DataSelector::City(new_city) => {
                self.location.city = new_city.to_string();
            }
            DataSelector::County(new_country) => {
                self.location.country = new_country.to_string();
            }
        }
    }
}

pub fn create_new_user(id: &str, name:&str, age: u8, location:Location ) -> UserData {
        UserData { id: id.to_string(), name: name.to_string(), age, location }
}

pub fn create_new_location(street: &str, postal_code: u16, city: &str, country: &str) -> Location {
    Location { street: street.to_string(), postal_code, city: city.to_string(), country: country.to_string() }
}