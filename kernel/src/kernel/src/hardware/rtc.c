#include <hardware/rtc.h>
#include <video/gop.h>
datetime_t current_datetime;

char * weekday_map[7] = {"Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"};
char * months_map[11] = {"Jan", "Feb", "Mar", "Apr", "May", "June", "July", "Aug", "Sep", "Oct", "Nov", "Dec"};
int is_updating_rtc() {
    outb(CMOS_ADDR, 0x0A);
    uint32_t status = inb(CMOS_DATA);
    return (status & 0x80);
}

uint8_t get_rtc_register(int reg_num) {
    outb(CMOS_ADDR, reg_num);
    return inb(CMOS_DATA);
}

void set_rtc_register(uint16_t reg_num, uint8_t val) {
    outb(CMOS_ADDR, reg_num);
    outb(CMOS_DATA, val);
}

datetime_t rtc_read_datetime() {
    while(is_updating_rtc());

    current_datetime.second = get_rtc_register(0x00);
    current_datetime.minute = get_rtc_register(0x02);
    current_datetime.hour = get_rtc_register(0x04);
    current_datetime.day = get_rtc_register(0x07);
    current_datetime.month = get_rtc_register(0x08);
    current_datetime.year = get_rtc_register(0x09);

    uint8_t registerB = get_rtc_register(0x0B);
    if (!(registerB & 0x04)) {
        current_datetime.second = (current_datetime.second & 0x0F) + ((current_datetime.second / 16) * 10);
        current_datetime.minute = (current_datetime.minute & 0x0F) + ((current_datetime.minute / 16) * 10);
        current_datetime.hour = ( (current_datetime.hour & 0x0F) + (((current_datetime.hour & 0x70) / 16) * 10) ) | (current_datetime.hour & 0x80);
        current_datetime.day = (current_datetime.day & 0x0F) + ((current_datetime.day / 16) * 10);
        current_datetime.month = (current_datetime.month & 0x0F) + ((current_datetime.month / 16) * 10);
        current_datetime.year = (current_datetime.year & 0x0F) + ((current_datetime.year / 16) * 10);
    }
    return current_datetime;
}

void rtc_write_datetime(datetime_t * dt) {
    while(is_updating_rtc());

    set_rtc_register(0x00, dt->second);
    set_rtc_register(0x02, dt->minute);
    set_rtc_register(0x04, dt->hour);
    set_rtc_register(0x07, dt->day);
    set_rtc_register(0x08, dt->month);
    set_rtc_register(0x09, dt->year);
}

char * datetime_to_str(datetime_t * dt) {
    char* ret = months_map[dt->month - 1];
    strcat(ret, " ");
    strcat(ret, weekday_map[get_weekday_from_date(dt)]);
    strcat(ret, "  ");
    strcat(ret, to_string64((uint64_t)dt->day));
    strcat(ret, " ");
    if(strlen(to_string64((uint64_t)dt->hour)) == 1){ strcat(ret, "0"); }
    strcat(ret, to_string64((uint64_t)dt->hour));
    strcat(ret, ":");
    if(strlen(to_string64((uint64_t)dt->minute)) == 1){ strcat(ret, "0"); }
    strcat(ret, to_string64((uint64_t)dt->minute));
    strcat(ret, ":");
    if(strlen(to_string64((uint64_t)dt->second)) == 1){ strcat(ret, "0"); }
    strcat(ret, to_string64((uint64_t)dt->second));
    strcat(ret, " 20");
    strcat(ret, to_string64((uint64_t)dt->year));
    return ret;
}

char * get_current_datetime_str() {
    return datetime_to_str(&current_datetime);
}

int get_weekday_from_date(datetime_t * dt) {
    char month_code_array[] = {0x0,0x3, 0x3, 0x6, 0x1, 0x4, 0x6, 0x2, 0x5, 0x0, 0x3, 0x5};
    char century_code_array[] = {0x4, 0x2, 0x0, 0x6, 0x4, 0x2, 0x0};    // Starting from 18 century
    dt->century = 21;
    int year_code = (dt->year + (dt->year / 4)) % 7;
    int month_code = month_code_array[dt->month - 1];
    int century_code = century_code_array[dt->century - 1 - 17];
    int leap_year_code = is_leap_year(dt->year, dt->month);

    int ret = (year_code + month_code + century_code + dt->day - leap_year_code) % 7;
    return ret;
}

int is_leap_year(int year, int month) {
    if(year % 4 == 0 && (month == 1 || month == 2)) return 1;
    return 0;
}

void rtc_init() {
    rtc_read_datetime();
}