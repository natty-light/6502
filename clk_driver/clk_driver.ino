#define clk 6 // d6
#define mode_btn 2 // d2
#define clk_btn 3 // d3
bool isManual = false;

void toggle_clk() {
  digitalWrite(clk, HIGH);
  delay(250);
  digitalWrite(clk, LOW);
  delay(250);
}

void mode_btn_int() {
  static unsigned long last_interrupt_time = 0;
  unsigned long interrupt_time = millis();
  if (interrupt_time - last_interrupt_time > 200) {
    isManual = !isManual;
  }
  last_interrupt_time = interrupt_time;
}

void clk_btn_int() {
  static unsigned long last_interrupt_time = 0;
  unsigned long interrupt_time = millis();
  if (interrupt_time - last_interrupt_time > 200) {
    toggle_clk();
  }
}

void setup() {
  pinMode(clk, OUTPUT);
  pinMode(mode_btn, INPUT);
  pinMode(clk_btn, INPUT);
  attachInterrupt(digitalPinToInterrupt(mode_btn), mode_btn_int, RISING);
  attachInterrupt(digitalPinToInterrupt(clk_btn), clk_btn_int, RISING);
}

void loop() {
  if (!isManual) {
    toggle_clk();
  } 
}
