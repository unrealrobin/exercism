#[derive(PartialEq, Debug)]
pub struct Clock{
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        //unimplemented!("Construct a new Clock from {hours} hours and {minutes} minutes");

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
        unimplemented!("Add {minutes} minutes to existing Clock time");
    }

    //convertings possible large minutes to hours and minutes (hours not rolled)
    pub fn minutes_to_hours_and_minutes(minutes_supplied: i32) -> Clock {

        if minutes_supplied < 60 {
            return Clock {hours: 0, minutes: minutes_supplied};
        }


        let minutes_left = minutes_supplied % 60; //returns only the remaining minutes
        let hours_left:i32 = (minutes_supplied - &minutes_left ) / 60;
    
        let x = Clock { 
            hours: hours_left, 
            minutes: minutes_left 
        };
    
        return x;
    }

    //Final Time after all adjustments
    pub fn set_time(hours: i32, minutes: i32) -> Clock{
        //26 hours -> 26%24 = 2
        let hours_corrected = {
            //rolling time
            if hours >= 24 {
                //get remainder or time less than 24 hours
                 let adjusted_hours = hours % 24;
                 //adjusting to a 12 hour clock (like AM/PM) -> 13 hours =  1
                 if adjusted_hours > 12 {
                     adjusted_hours - 12
                 }else{
                     adjusted_hours
                 }
            }else{
                 let adjusted_hours = hours;
                 if adjusted_hours > 12 {
                     adjusted_hours - 12
                 }else{
                     adjusted_hours
                 }
            }
         };

         let y = Clock{hours: hours_corrected, minutes};

         y
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