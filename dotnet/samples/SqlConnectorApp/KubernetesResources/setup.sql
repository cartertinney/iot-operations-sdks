    PRINT 'Starting setup script';
    USE [master];
    GO
    IF NOT EXISTS (SELECT * FROM sys.databases WHERE name = 'MySampleDB')
    BEGIN
        CREATE DATABASE MySampleDB;
        PRINT 'Created MySampleDB database';
    END
    ELSE
    BEGIN
        PRINT 'MySampleDB database already exists';
    END
    GO
    USE MySampleDB;
    PRINT 'Switched to MySampleDB database';
    GO
    IF NOT EXISTS (SELECT * FROM sys.tables WHERE name = 'CountryMeasurements')
    BEGIN
        CREATE TABLE CountryMeasurements (
            ID INT PRIMARY KEY IDENTITY(1,1),
            Country CHAR(2),
            Viscosity DECIMAL(3,2),
            Sweetness DECIMAL(3,2),
            ParticleSize DECIMAL(3,2),
            Overall DECIMAL(3,2)
        );
        INSERT INTO CountryMeasurements (Country, Viscosity, Sweetness, ParticleSize, Overall)
        VALUES
            ('us', 0.50, 0.80, 0.70, 0.40),
            ('fr', 0.60, 0.85, 0.75, 0.45),
            ('jp', 0.53, 0.83, 0.73, 0.43),
            ('uk', 0.51, 0.81, 0.71, 0.41);
        PRINT 'Created and populated CountryMeasurements table';
    END
    ELSE
    BEGIN
        PRINT 'CountryMeasurements table already exists';
    END
    GO
    PRINT 'Setup script completed';