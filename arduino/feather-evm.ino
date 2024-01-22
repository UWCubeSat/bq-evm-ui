#include <Wire.h>

uint16_t read_from_msp(unsigned int reg, int size) {
  // This is more "standard" I2C.
  // From my experience, it usually follow the flow below.
  // 
  // Send Read Address.
  // Send Read Register.
  // Send Start Again (which is what the endTransmission(false) is doing)
  // Send Read Address Again.
  // THEN the peripheral I2C device responds with the data.
  
  uint8_t msp_address = 0x40;
  Wire.beginTransmission(msp_address);
  Wire.write(reg);
  Wire.endTransmission(false);
  Wire.requestFrom(msp_address, 1);

  unsigned int returnInt = Wire.read();
  for(int i = 0; i < size-1; i++) {
    returnInt = (returnInt << 8) | Wire.read();
  }

  return returnInt;
}

uint16_t read_from_bq(unsigned int reg, int size) {
  // Read from BQ76925
  // From: https://www.ti.com/lit/ds/symlink/bq76925.pdf?ts=1705535736818
  // I2C is not really standardized so things happen however chip manus want it to.
  // This chip encodes both the I2C address AND the register into one address.
  uint8_t bq_group_addr = 0b0100;
  uint8_t bq_address = (bq_group_addr << 3) + reg;
  Wire.beginTransmission(bq_address);
  Wire.endTransmission(false);
  Wire.requestFrom((bq_address), size);

  unsigned int returnInt = Wire.read();
  for(int i = 0; i < size-1; i++) {
    returnInt = (returnInt << 8) | Wire.read();
  }

  return returnInt;
}

void write_to_bq(unsigned int reg, uint8_t data) {
  // Read from BQ76925
  // From: https://www.ti.com/lit/ds/symlink/bq76925.pdf?ts=1705535736818
  // I2C is not really standardized so things happen however chip manus want it to.
  // This chip encodes both the I2C address AND the register into one address.
  uint8_t bq_group_addr = 0b0100;
  uint8_t bq_address = ((bq_group_addr << 3) + reg);
  Wire.beginTransmission(bq_address);
  Wire.write(data);
  Wire.endTransmission();
}

int get_device_from_params(String command_params) {
  String device = command_params.substring(0, command_params.indexOf(' '));

  int value = device.compareTo("msp");
  return value == 0 ? 0 : 1;
}

int get_register_from_params(String command_params) {
  String sub = command_params.substring(command_params.indexOf(' '));
  String reg = sub.substring(0, command_params.indexOf(' '));
  return reg.toInt();
}

int get_third_from_params(String command_params) {
  String sub = command_params.substring(command_params.indexOf(' '));
  String third = sub.substring(command_params.indexOf(' ')+1);
  return third.toInt();
}

void setup() {
  while ( !Serial.available() ) { delay( 1 ); }
  Serial.begin(9600);
  Wire.begin();
  Wire.setClock(10000);
}

char input[128] = "";
int i = 0;

void loop() {

  while(Serial.available()) {
    char x = Serial.read();
    input[i] = x;
    if (x == '\n')
    {
      input[i] = 0x00;
      String command = String(input);
      String command_head = command.substring(0, command.indexOf(' '));
      String command_params = command.substring(command.indexOf(' ') + 1);
      
      unsigned int device = get_device_from_params(command_params);
      unsigned int reg = get_register_from_params(command_params);
      unsigned int third = get_third_from_params(command_params);

      if(command_head.compareTo("read") == 0) {
        if(device) {
          // BQ
          unsigned int value = read_from_bq(reg, third);
          Serial.println(value);
        } else {
          // MSP
          unsigned int value = read_from_msp(reg, third);
          Serial.println(value);
        }

      } else if(command_head.compareTo("write") == 0){
        if(device) {
          write_to_bq(reg, third);
          Serial.println("SUCCESS!");
        } else {
          Serial.println("Invalid Command | Only the 'bq' device is enable for 'write'!");
        }
      } else {
        Serial.println("Invalid Command | Only 'read' or 'write' is allowed!");
      }
      i = 0;
    } else {
      i++;
    }
  }
}

