<?php
$servername = "mysql100.unoeuro.com";
$port = 3306;
$username = "nexus_energy_dk";
$password = "erA46bR5G2t3awzhm9xf";
$database = "nexus_energy_dk_db_p4";

if ($_SERVER['REQUEST_METHOD'] === 'POST') {
    $data = file_get_contents('php://input');
    file_put_contents('log1.txt', $data, FILE_APPEND);
    $decodedData = json_decode($data, true);

    $conn = new mysqli($servername, $username, $password, $database, $port);
    if ($conn->connect_error) {
        die("Connection failed: " . $conn->connect_error);
    }

    $sql = "INSERT INTO iot_data (";
    $placeholders = "";
    $values = [];

    foreach ($decodedData as $key => $value) {
        if ($key === 'gws' && is_array($value)) {
            foreach ($value as $gwsData) {
                foreach ($gwsData as $nestedKey => $nestedValue) {
                    if ($nestedKey !== 'ts') {
                        $sql .= "`$nestedKey`, ";
                        $placeholders .= "?, ";
                        $values[] = $nestedValue;
                    }
                }
            }
        } else {
            $sql .= "`$key`, ";
            $placeholders .= "?, ";
            $values[] = $value;
        }
    }

    $sql = rtrim($sql, ", ") . ") VALUES (" . rtrim($placeholders, ", ") . ")";
    $stmt = $conn->prepare($sql);

    if (!$stmt) {
        die("Error preparing statement: " . $conn->error);
    }

    $types = str_repeat('s', count($values));
    $stmt->bind_param($types, ...$values);

    if (!$stmt->execute()) {
        die("Error executing statement: " . $stmt->error);
    }

    $stmt->close();
    $conn->close();
}

echo "Data received.";
?>
