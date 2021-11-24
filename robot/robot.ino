#include <LiquidCrystal.h>

// initialize the library by associating any needed LCD interface pin
// with the arduino pin number it is connected to
const int rs = 12, en = 11, d4 = 5, d5 = 4, d6 = 3, d7 = 2;
LiquidCrystal lcd(rs, en, d4, d5, d6, d7);

char buffer[100];

void setup() {
  Serial.begin(9600);
  Serial.setTimeout(10);
  // set up the LCD's number of columns and rows:
  lcd.begin(16, 2);
  // Print a message to the LCD.
  lcd.print("hello, world!");
}


void loop() {
  lcd.clear();
  // set the cursor to column 0, line 1
  // (note: line 1 is the second row, since counting begins with 0):
  lcd.setCursor(0, 0);

  size_t bytes_read = Serial.readBytesUntil('!', buffer, 100);

  if (bytes_read == 0) {
    lcd.print("Nothing read");
  }
  else if (bytes_read >= 100) {
    lcd.print("Como assim");
  }
  else {
    buffer[bytes_read] = '\0';
    lcd.print("What i got");
    lcd.setCursor(0,1);
    lcd.print(buffer);
    
  }
  

  delay(500);
}
