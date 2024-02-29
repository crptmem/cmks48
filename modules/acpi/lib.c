#include "lib.h"

void exit(uint32_t code) {
  asm volatile(
    "movq $0x00, %%rbx\r\n"
    "movq $0x50, %%rcx\r\n"
    "int $0x80" 
    ::: "rdi", "rbx");
}

uint32_t strlen(const char *str)
{
	const char *s;
	for (s = str; *s; ++s)
		;
	return (s - str);
}

char * itoa( int value, char * str, int base )
{
    char * rc;
    char * ptr;
    char * low;
    // Check for supported base.
    if ( base < 2 || base > 36 )
    {
        *str = '\0';
        return str;
    }
    rc = ptr = str;
    // Set '-' for negative decimals.
    if ( value < 0 && base == 10 )
    {
        *ptr++ = '-';
    }
    // Remember where the numbers start.
    low = ptr;
    // The actual conversion.
    do
    {
        // Modulo is negative for negative value. This trick makes abs() unnecessary.
        *ptr++ = "zyxwvutsrqponmlkjihgfedcba9876543210123456789abcdefghijklmnopqrstuvwxyz"[35 + value % base];
        value /= base;
    } while ( value );
    // Terminating the string.
    *ptr-- = '\0';
    // Invert the numbers.
    while ( low < ptr )
    {
        char tmp = *low;
        *low++ = *ptr;
        *ptr-- = tmp;
    }
    return rc;
}

int vsprintf(char* buffer, const char* format, va_list args) {
    int count = 0;  // Count of characters written to the buffer
    char temp[20];  // Temporary buffer for number-to-string conversion

    // Iterate through the format string
    while (*format != '\0') {
        if (*format == '%') {
            format++;  // Move past '%'
            char specifier = *format;

            switch (specifier) {
                case 'd': {
                    // Integer
                    int value = va_arg(args, int);
                    itoa(value, temp, 10);
                    int i = 0;
                    while (temp[i] != '\0') {
                        buffer[count++] = temp[i++];
                    }
                    break;
                }
                case 'x': {
                    // Integer
                    int value = va_arg(args, int);
                    itoa(value, temp, 16);
                    int i = 0;
                    while (temp[i] != '\0') {
                        buffer[count++] = temp[i++];
                    }
                    break;
                }
                case 's': {
                    // String
                    char* value = va_arg(args, char*);
                    int i = 0;
                    while (value[i] != '\0') {
                        buffer[count++] = value[i++];
                    }
                    break;
                }
                // Add more cases for other specifiers as needed

                default:
                    // Unsupported specifier, just copy it as is
                    buffer[count++] = '%';
                    buffer[count++] = specifier;
                    break;
            }
        } else {
            // Regular character, copy it to the buffer
            buffer[count++] = *format;
        }

        format++;  // Move to the next character in the format string
    }

    // Add null terminator to the resulting string
    buffer[count] = '\0';

    return count;
}

int memcmp(const void *v1, const void *v2, unsigned long n)
{
    const char *s1 = (const char *)v1;
    const char *s2 = (const char *)v2;
    while (n-- > 0)
    {
        if (*s1 != *s2)
        {
            return (int)((unsigned char)*s1 - (unsigned char)*s2);
        }
        s1++, s2++;
    }
    return 0;
}

