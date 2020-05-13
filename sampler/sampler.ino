/* Arduino code to program AVR to sample ADC/DAC at 4 kHz
 * Zack Johnson 13 May 2020
 */

#include <SPI.h>

const int adcPin = 20;
SPISettings settingsA(32000, MSBFIRST, SPI_MODE0);

ISR(TIMER1_COMPA_vect) {	// interrupt sampling function
  SPI.beginTransaction(settingsA);
  digitalWrite(adcPin, LOW);
  SPI.transfer( 0x01 );
  uint16_t adcVal = SPI.transfer16( 0x8000 ) & 0x03FF;
  digitalWrite(adcPin, HIGH);
  Serial.write(adcVal);
}

void setup() {
  cli();	// stop interrupts

  // configure serial interface
  Serial.begin(9600);

  // initialize SPI
  pinMode( adcPin, OUTPUT );
  SPI.begin();

  // set timer1 interrupt for 4 kHz
  // ! modified to 1 Hz
  TCCR1A = 0;				// set entire TCCR1A register to 0
  TCCR1B = 0;				// same for TCCR1B
  TCNT1  = 0;				// initialize counter value to 0
  OCR1A	 = 15624;			// count to interrupt (1 Hz)
  //OCR1A = 499;			// count to interrupt (4 kHz)
  TCCR1B |= (1 << WGM12);	// turn on CTC mode
  TCCR1B |= (1 << CS12) | (1 << CS10);	// set for 1024 prescaler
  //TCCR1B |= (1 << CS11);	// se for 8 prescaler
  TIMSK1 |= (1 << OCIE1A);	// enable timer compare interrupt

  sei();	// allow interrupts
}

void loop() {
  while( true ) {}
}
