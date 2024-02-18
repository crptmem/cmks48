#include "lib.h"
#define PORT 0x3f8

uint32_t inl(uint16_t portid)
{
	uint32_t ret;
	asm volatile("inl %%dx, %%eax":"=a"(ret):"d"(portid));
	return ret;
}

void outw(uint16_t portid, uint16_t value)
{
	asm volatile("outw %%ax, %%dx": :"d" (portid), "a" (value));
}
void outl(uint16_t portid, uint32_t value)
{
	asm volatile("outl %%eax, %%dx": :"d" (portid), "a" (value));
}

static inline void outb(uint16_t port, uint8_t val)
{
    __asm__ volatile ( "outb %b0, %w1" : : "a"(val), "Nd"(port) : "memory");
    /* There's an outb %al, $imm8 encoding, for compile-time constant port numbers that fit in 8b. (N constraint).
     * Wider immediate constants would be truncated at assemble-time (e.g. "i" constraint).
     * The  outb  %al, %dx  encoding is the only option for all other cases.
     * %1 expands to %dx because  port  is a uint16_t.  %w1 could be used if we had the port number a wider C type */
}

static inline uint8_t inb(uint16_t port)
{
    uint8_t ret;
    __asm__ volatile ( "inb %w1, %b0"
                   : "=a"(ret)
                   : "Nd"(port)
                   : "memory");
    return ret;
}

int is_transmit_empty() {
   return inb(PORT + 5) & 0x20;
}
 
void write_serial(char a) {
   while (is_transmit_empty() == 0);
 
   outb(PORT,a);
}

uint32_t
strlen(const char *str)
{
	const char *s;
	for (s = str; *s; ++s)
		;
	return (s - str);
}

void write_string(char *str) {
  const int len = strlen(str);
  for(int i = 0; i < len; i++) {
    write_serial(str[i]);
  }
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

int mprintf(const char *str, ...) {
  if (!str)
    return 0;
  va_list ap;
  char buf[4096];
  va_start(ap, str);

  vsprintf(buf, str, ap);

  va_end(ap);
  write_string(buf);
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

