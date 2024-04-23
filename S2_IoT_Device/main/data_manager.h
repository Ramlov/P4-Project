#ifndef DATA_MANAGER_H
#define DATA_MANAGER_H

#include <Arduino.h>

class DataManager {
public:
    DataManager();  // Constructor
    ~DataManager(); // Destructor

    void setData(int newData);
    int getData() const;

private:
    int data;
};

#endif // DATA_MANAGER_H