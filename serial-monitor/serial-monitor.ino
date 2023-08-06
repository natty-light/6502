const char ADDR[] = {22, 24, 26, 28, 30, 32, 34, 36, 38, 40, 42, 44, 46, 48, 50, 52};
const char DATA[] = {39, 41, 43, 45, 47, 49, 51, 53};
#define CLOCK 2
#define RWB 3


void onClock() {
  unsigned int addr = 0;
  unsigned int data = 0;
  char output[15] = "";

  for (int i = 15; i >= 0; i--) {
    int bit = digitalRead(ADDR[i] ? 1 : 0);
    Serial.print(bit);
    addr = (addr << i) + bit;
  }

  Serial.print("   ");
  
  for (int i = 7; i >= 0; i--) {
    int bit = digitalRead(DATA[i] ? 1 : 0);
    data = (data << i) + bit;
    Serial.print(bit);
  }
  sprintf(output, " %04x %c %02x", addr, digitalRead(RWB) ? 'r' : 'w', data);
  Serial.println(output);
}

void setup() {
  for (int i = 0; i < 16; i++) {
    pinMode(ADDR[i], INPUT);
  }
  for (int i = 0; i < 8; i++) {
    pinMode(DATA[i], INPUT);
  }
  pinMode(CLOCK, INPUT);
  pinMode(RWB, INPUT);

  attachInterrupt(digitalPinToInterrupt(CLOCK), onClock, RISING);
  Serial.begin(57600);
}

void loop() {
  // put your main code here, to run repeatedly:

}

