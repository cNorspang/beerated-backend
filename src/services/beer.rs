use crate::routes::beer::Beer; 

pub fn get_beer_by_id(beer_id: u32) -> Beer {
    let id = beer_id;
    let name = "Westmalle Tripple";
    let brewery = "Brouwerij der Trappisten van Westmalle";
    let submitter_id = 1;
    let alcohol_content = 9.5;
    let score = 8.2;

    let beer = Beer {
        id,
        name: name.to_string(),
        brewery: brewery.to_string(),
        submitter_id,
        alcohol_content,
        score
    };

    beer
}  
