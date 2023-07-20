#define clk 6 // d6
#define mode_btn 2 // d2
#define clk_btn 3 // d3
#define pulse_period 250
#define milli_to_micro 1000

bool is_manual = false;
bool manual_clock_set = false;

void toggle_clk() {
  digitalWrite(clk, HIGH);
  delay(pulse_period);
  digitalWrite(clk, LOW);
  delay(pulse_period);
}

void mode_btn_int() {
  static unsigned long last_interrupt_time = 0;
  unsigned long interrupt_time = millis();
  if (interrupt_time - last_interrupt_time > 200) {
    is_manual = !is_manual;
  }
  last_interrupt_time = interrupt_time;
}

void clk_btn_int() {
  static unsigned long last_interrupt_time = 0;
  unsigned long interrupt_time = millis();
  if (interrupt_time - last_interrupt_time > 200) {
    manual_clock_set = true;
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
  if (!is_manual) {
    toggle_clk();
  } else if (manual_clock_set) {
    toggle_clk();
    manual_clock_set = false;
  }
}
