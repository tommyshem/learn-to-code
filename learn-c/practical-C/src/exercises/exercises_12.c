#include <stdio.h>

/* Exercise 12_1 write structure mailing list and print it out */
void exercise_12_1() {
  struct mailing {
    char name[60];     /* last name first then first name*/
    char address1[60]; /* two lines of address*/
    char address2[60];
    char city[40];     /* city name */
    char state[2];     /* abbreviation for state */
    long int zip_code; /* number zip code*/
  };

  /* mailing list with 2 items */
  struct mailing mailing_list[2] = {{"Smith John", "23 Fix street", "Fixer",
                                     "New England", "NE", 57689477477},
                                    {"Chase Jane", "24 Frozen street", "Frozen",
                                     "North Wing", "NW", 57689477477}};
  for (int index = 0; index < 2; ++index) {
    printf("Name %s\n", mailing_list[index].name);
    printf("Address %s\n", mailing_list[index].address1);
    printf("Address %s\n", mailing_list[index].address2);
    printf("City %s\n", mailing_list[index].city);
    printf("State %s\n", mailing_list[index].state);
    printf("Zip Code %ld\n", mailing_list[index].zip_code);
  }
}

/* Exercise 12_2 design a structure to store time and date write a function to
 * find the difference between the to times in minutes*/
void exercise_12_2() {

  typedef struct _time_date {
    int Year;
    int Month;
    int Day;
    int Hour;
    int Minute;
    int Second;
    int Milliseconds;
  } time_date;

  struct _time_date date1 = {.Year = 2024,
                             .Month = 12,
                             .Day = 24,
                             .Hour = 12,
                             .Minute = 24,
                             .Second = 34,
                             .Milliseconds = 500};

  char string[20];
  /* time */
  snprintf(string, sizeof(string), "%d:%d:%d", date1.Hour, date1.Minute,
           date1.Second);
  printf("Time = %s\n", string);
  /* Date */
  snprintf(string, sizeof(string), "%d/%d/%d", date1.Day, date1.Month,
           date1.Year);
  printf("Date = %s\n", string);
}
/* Exercise 12_3 design structure with flight number, 3 character airport code,3
 * character airport destination,starting time,arrival time */
void exercise_12_3() {

  typedef struct _flight_plan {
    int Flight_number;
    char airport_code[4];
    char airport_destination_code[4];
    int Year;
    int Month;
    int Day;
    int Departure_Hour;
    int Departure_Minute;
    int Arrival_Hour;
    int Arrival_Minute;
    int Flying_time;

  } flight_plan;
  /* Variables */
  struct _flight_plan flight_list = {
      .Flight_number = 45,
      .airport_code = "ERT",
      .airport_destination_code = "YTR",
      .Year = 2024,
      .Month = 12,
      .Day = 27,
      .Departure_Hour = 12,
      .Departure_Minute = 53,
      .Arrival_Hour = 13,
      .Arrival_Minute = 55,
      .Flying_time = 62,
  };
  /* get time difference */
  int minutes = 0;
  int hours = 0;
  const int minutes_per_hour = 60; /* minutes in a hour */
  /* calculate total minutes flying */
  if (flight_list.Departure_Minute > flight_list.Arrival_Minute) {
    minutes = minutes_per_hour - flight_list.Departure_Minute;
    minutes = minutes + flight_list.Arrival_Minute;
    hours = flight_list.Arrival_Hour - flight_list.Departure_Hour;
    hours = hours - 1;
    minutes = minutes + (hours * minutes_per_hour);
  } else {
    minutes = flight_list.Arrival_Minute - flight_list.Departure_Minute;
    hours = flight_list.Arrival_Hour - flight_list.Departure_Hour;
    minutes = minutes + (hours * minutes_per_hour);
  }
  printf("Flying Time = %i\n", minutes);
  printf("Estimated Flying Time = %i\n", flight_list.Flying_time);
}
/* Exercise 12_4 list all the planes that leave from the same two airports from
 * the list with a given airport code from std input */
void exercise_12_4() {
  /* structure of a flight plan */
  typedef struct _flight_plan {
    int Flight_number;
    char airport_code[4];
    char airport_destination_code[4];
  } flight_plan;

  /* Variables */
  struct _flight_plan flight_list[6] = {{
                                            .Flight_number = 45,
                                            .airport_code = "ERT",
                                            .airport_destination_code = "YTR",
                                        },
                                        {
                                            .Flight_number = 47,
                                            .airport_code = "RYP",
                                            .airport_destination_code = "HYT",
                                        },
                                        {
                                            .Flight_number = 48,
                                            .airport_code = "ERT",
                                            .airport_destination_code = "PUT",
                                        },
                                        {
                                            .Flight_number = 49,
                                            .airport_code = "ERT",
                                            .airport_destination_code = "RYP",
                                        },
                                        {
                                            .Flight_number = 50,
                                            .airport_code = "CRT",
                                            .airport_destination_code = "YTR",
                                        },
                                        {
                                            .Flight_number = 12,
                                            .airport_code = "PUT",
                                            .airport_destination_code = "YTR",
                                        }};
  /* sort airports  */
  printf("Input airport code to search from these airport codes PUT CRT ERT RYP : ");
  
  char airport_code[4];
 fgets(airport_code, sizeof(airport_code), stdin);
 
 printf("Input = %s\n",airport_code);
 
 

 printf("Airport code : %s",flight_list[1].airport_code);
 
 }

