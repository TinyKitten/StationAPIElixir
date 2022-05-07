-- MySQL dump 10.13  Distrib 8.0.28, for macos12.0 (arm64)
--
-- Host: localhost    Database: stationapi
-- ------------------------------------------------------
-- Server version	8.0.28

/*!40101 SET @OLD_CHARACTER_SET_CLIENT=@@CHARACTER_SET_CLIENT */;
/*!40101 SET @OLD_CHARACTER_SET_RESULTS=@@CHARACTER_SET_RESULTS */;
/*!40101 SET @OLD_COLLATION_CONNECTION=@@COLLATION_CONNECTION */;
/*!50503 SET NAMES utf8mb4 */;
/*!40103 SET @OLD_TIME_ZONE=@@TIME_ZONE */;
/*!40103 SET TIME_ZONE='+00:00' */;
/*!40014 SET @OLD_UNIQUE_CHECKS=@@UNIQUE_CHECKS, UNIQUE_CHECKS=0 */;
/*!40014 SET @OLD_FOREIGN_KEY_CHECKS=@@FOREIGN_KEY_CHECKS, FOREIGN_KEY_CHECKS=0 */;
/*!40101 SET @OLD_SQL_MODE=@@SQL_MODE, SQL_MODE='NO_AUTO_VALUE_ON_ZERO' */;
/*!40111 SET @OLD_SQL_NOTES=@@SQL_NOTES, SQL_NOTES=0 */;

--
-- Table structure for table `companies`
--

DROP TABLE IF EXISTS `companies`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `companies` (
  `company_cd` int unsigned NOT NULL,
  `rr_cd` int unsigned NOT NULL,
  `company_name` varchar(14) CHARACTER SET utf8 COLLATE utf8_general_ci NOT NULL DEFAULT '',
  `company_name_k` varchar(17) CHARACTER SET utf8 COLLATE utf8_general_ci NOT NULL DEFAULT '',
  `company_name_h` varchar(17) CHARACTER SET utf8 COLLATE utf8_general_ci NOT NULL DEFAULT '',
  `company_name_r` varchar(11) CHARACTER SET utf8 COLLATE utf8_general_ci NOT NULL DEFAULT '',
  `company_name_en` varchar(36) CHARACTER SET utf8 COLLATE utf8_general_ci DEFAULT '',
  `company_url` varchar(47) CHARACTER SET utf8 COLLATE utf8_general_ci NOT NULL DEFAULT '',
  `company_type` varchar(1) CHARACTER SET utf8 COLLATE utf8_general_ci NOT NULL DEFAULT '',
  `e_status` int unsigned NOT NULL,
  `e_sort` int unsigned NOT NULL,
  PRIMARY KEY (`company_cd`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb3;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `companies`
--

LOCK TABLES `companies` WRITE;
/*!40000 ALTER TABLE `companies` DISABLE KEYS */;
/*!40000 ALTER TABLE `companies` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `lines`
--

DROP TABLE IF EXISTS `lines`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `lines` (
  `line_cd` int unsigned NOT NULL,
  `company_cd` int unsigned NOT NULL,
  `line_name` varchar(19) NOT NULL,
  `line_name_k` varchar(21) NOT NULL,
  `line_name_h` varchar(19) NOT NULL,
  `line_name_r` varchar(52) NOT NULL,
  `line_color_c` varchar(9) DEFAULT NULL,
  `line_color_t` varchar(10) NOT NULL,
  `line_type` int unsigned NOT NULL,
  `lon` decimal(18,14) unsigned NOT NULL,
  `lat` decimal(18,15) unsigned NOT NULL,
  `zoom` int unsigned NOT NULL,
  `e_status` int unsigned NOT NULL,
  `e_sort` int unsigned NOT NULL,
  PRIMARY KEY (`line_cd`),
  KEY `company_cd` (`company_cd`),
  CONSTRAINT `lines_ibfk_1` FOREIGN KEY (`company_cd`) REFERENCES `companies` (`company_cd`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb3;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `lines`
--

LOCK TABLES `lines` WRITE;
/*!40000 ALTER TABLE `lines` DISABLE KEYS */;
/*!40000 ALTER TABLE `lines` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `station_station_types`
--

DROP TABLE IF EXISTS `station_station_types`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `station_station_types` (
  `id` int unsigned NOT NULL AUTO_INCREMENT,
  `station_cd` int unsigned NOT NULL,
  `type_cd` int unsigned NOT NULL,
  `line_group_cd` int unsigned NOT NULL,
  `pass` int unsigned NOT NULL DEFAULT '0',
  PRIMARY KEY (`id`),
  KEY `type_cd` (`type_cd`),
  KEY `id` (`id`),
  KEY `station_cd` (`station_cd`),
  CONSTRAINT `station_station_types_ibfk_1` FOREIGN KEY (`station_cd`) REFERENCES `stations` (`station_cd`),
  CONSTRAINT `station_station_types_ibfk_2` FOREIGN KEY (`type_cd`) REFERENCES `types` (`type_cd`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb3;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `station_station_types`
--

LOCK TABLES `station_station_types` WRITE;
/*!40000 ALTER TABLE `station_station_types` DISABLE KEYS */;
/*!40000 ALTER TABLE `station_station_types` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `stations`
--

DROP TABLE IF EXISTS `stations`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `stations` (
  `station_cd` int unsigned NOT NULL,
  `station_g_cd` int unsigned NOT NULL,
  `station_name` text CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL,
  `station_name_k` text CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL,
  `station_name_r` text CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL,
  `station_name_zh` text CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL,
  `station_name_ko` text CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL,
  `line_cd` int unsigned NOT NULL,
  `pref_cd` int unsigned NOT NULL,
  `post` text CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL,
  `address` text CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL,
  `lon` decimal(11,7) unsigned NOT NULL,
  `lat` decimal(11,8) unsigned NOT NULL,
  `open_ymd` text CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL,
  `close_ymd` text CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL,
  `e_status` int unsigned NOT NULL,
  `e_sort` int unsigned NOT NULL,
  KEY `station_g_cd` (`station_g_cd`),
  KEY `line_cd` (`line_cd`),
  KEY `coordinates` (`lat`,`lon`),
  KEY `station_cd` (`station_cd`),
  CONSTRAINT `stations_ibfk_1` FOREIGN KEY (`line_cd`) REFERENCES `lines` (`line_cd`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `stations`
--

LOCK TABLES `stations` WRITE;
/*!40000 ALTER TABLE `stations` DISABLE KEYS */;
/*!40000 ALTER TABLE `stations` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `types`
--

DROP TABLE IF EXISTS `types`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `types` (
  `type_cd` int unsigned NOT NULL AUTO_INCREMENT,
  `type_name` varchar(19) CHARACTER SET utf8 COLLATE utf8_general_ci NOT NULL DEFAULT '',
  `type_name_k` varchar(14) CHARACTER SET utf8 COLLATE utf8_general_ci NOT NULL DEFAULT '',
  `type_name_r` varchar(53) CHARACTER SET utf8 COLLATE utf8_general_ci NOT NULL DEFAULT '',
  `type_name_zh` varchar(19) CHARACTER SET utf8 COLLATE utf8_general_ci NOT NULL DEFAULT '',
  `type_name_ko` varchar(25) CHARACTER SET utf8 COLLATE utf8_general_ci NOT NULL DEFAULT '',
  `color` varchar(7) CHARACTER SET utf8 COLLATE utf8_general_ci NOT NULL DEFAULT '',
  `direction` int unsigned NOT NULL DEFAULT '0',
  PRIMARY KEY (`type_cd`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb3;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `types`
--

LOCK TABLES `types` WRITE;
/*!40000 ALTER TABLE `types` DISABLE KEYS */;
/*!40000 ALTER TABLE `types` ENABLE KEYS */;
UNLOCK TABLES;
/*!40103 SET TIME_ZONE=@OLD_TIME_ZONE */;

/*!40101 SET SQL_MODE=@OLD_SQL_MODE */;
/*!40014 SET FOREIGN_KEY_CHECKS=@OLD_FOREIGN_KEY_CHECKS */;
/*!40014 SET UNIQUE_CHECKS=@OLD_UNIQUE_CHECKS */;
/*!40101 SET CHARACTER_SET_CLIENT=@OLD_CHARACTER_SET_CLIENT */;
/*!40101 SET CHARACTER_SET_RESULTS=@OLD_CHARACTER_SET_RESULTS */;
/*!40101 SET COLLATION_CONNECTION=@OLD_COLLATION_CONNECTION */;
/*!40111 SET SQL_NOTES=@OLD_SQL_NOTES */;

-- Dump completed on 2022-05-06 19:12:08
