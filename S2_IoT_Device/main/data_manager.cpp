#include "data_manager.h"

DataManager::DataManager() {
    data = 0; // Initialize data to 0
}

DataManager::~DataManager() {
    // Destructor
}

void DataManager::setData(int newData) {
    data = newData;
}

int DataManager::getData() const {
    return data;
}