#[derive(PartialEq, Debug)]
pub struct Clock{
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        

        //getting the total hours and minutes from minutes (Not Rolled Hours)
        let from_minutes = Clock::minutes_to_hours_and_minutes(minutes);
        //Getting Hours total
        let total_hours: i32 = from_minutes.hours + hours;
        //Getting Minutes Total
        let remaining_minutes:i32 = from_minutes.minutes;

        let total_time = Clock::set_time(total_hours, remaining_minutes); //Clock{hours: hours_corrected, minutes};

        Clock { hours: total_time.hours, minutes: total_time.minutes }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        
        return Clock::new(self.hours, self.minutes + minutes);

    }

    //convertings possible large minutes to hours and minutes (hours not rolled)
    pub fn minutes_to_hours_and_minutes(minutes_supplied: i32) -> Clock {

        let mut minutes_left= 0;
        let mut hours_left= 0;

        match minutes_supplied {
            //if minutes dont need to roll
            minutes_supplied if minutes_supplied < 60 && minutes_supplied >= 0 => {
                minutes_left = minutes_supplied;
                hours_left = 0;
            },
            minutes_supplied if minutes_supplied > 60 => { //if need to roll minutes
                minutes_left = minutes_supplied % 60; //returns only the remaining minutes
                hours_left = (minutes_supplied - minutes_left ) / 60;
            },
            minutes_supplied if minutes_supplied < 0 => { //if negative minutes
                //minutes =  -5
                hours_left = hours_left - (minutes_left.abs() / 60); //subtracts the amount of negative hours from hours left.
                minutes_left = 60 - (minutes_supplied.abs() % 60);
            },
            _ => {
                minutes_left = 0;
                hours_left = 0;
            }
        }
        let x = Clock {hours: hours_left, minutes: minutes_left };
        return x;
    }

    //Final Time after all adjustments
    pub fn set_time(hours: i32, minutes: i32) -> Clock {

        let adjusted_hours;


        match hours {
            hours if hours >= 24 => { //rollings hours and correcting 12hr clock
                adjusted_hours = Clock::correct_hours_to_12hr_clock(hours % 24);

            },
            hours if hours > 0 && hours < 24 => { //Correcting 12 hour clock
                adjusted_hours = Clock::correct_hours_to_12hr_clock(hours);
            },
            hours if hours < 0 => {
                adjusted_hours = Clock::correct_hours_to_12hr_clock(24 - (hours.abs() % 24));
            },
            _ => adjusted_hours = 0
        }

         let y = Clock{hours: adjusted_hours, minutes};

         y
    }

    pub fn correct_hours_to_12hr_clock(hours: i32) -> i32 {

        match hours {
            hours if hours > 12 => {
                let hrs = hours - 12;
                hrs
            },
            hours if hours <= 12 => {
                hours
            },
            _ => hours,
        }
    }
}

impl ToString for Clock {

    fn to_string(&self) -> String {

        //do check strings to add the 0 before if digit is less than 10
        let formatted_hours = {
            if self.hours < 10 {
                format!("0{}", &self.hours)
            }else{
                format!("{}", &self.hours)
            }
        };

        let formatted_minutes = {
            if self.minutes < 10 {
                format!("0{}", &self.minutes)
            }else{
                format!("{}", &self.minutes)
            }
        };


        format!("{}:{}", formatted_hours, formatted_minutes)
    }

}