import { Injectable } from '@nestjs/common';
import { RowDataPacket } from 'mysql2';
import { LineRepository } from 'src/line/line.repository';
import { LineRaw } from 'src/line/models/LineRaw';
import { MysqlService } from 'src/mysql/mysql.service';
import { StationRaw } from 'src/station/models/StationRaw';
import { TrainTypeRaw, TrainTypeWithLineRaw } from './models/TrainTypeRaw';

@Injectable()
export class TrainTypeRepository {
  constructor(
    private readonly mysqlService: MysqlService,
    private lineRepo: LineRepository,
  ) {}

  async findOne(lineGroupId: number): Promise<TrainTypeRaw> {
    const { connection } = this.mysqlService;

    return new Promise<TrainTypeRaw>((resolve, reject) => {
      connection.query(
        `SELECT *
        FROM types as t, station_station_types as sst
        WHERE sst.line_group_cd = ?
          AND t.type_cd = sst.type_cd
        LIMIT 1`,
        [lineGroupId],
        (err, results: RowDataPacket[]) => {
          if (err) {
            return reject(err);
          }
          if (!results.length) {
            return resolve(null);
          }
          return resolve(results[0] as TrainTypeRaw);
        },
      );
    });
  }

  async getBelongingStations(
    lineGroupId: number,
    excludePass?: boolean,
  ): Promise<StationRaw[]> {
    const { connection } = this.mysqlService;

    return new Promise<StationRaw[]>((resolve, reject) => {
      connection.query(
        `SELECT *
        FROM station_station_types as sst, stations as s
        WHERE sst.line_group_cd = ?
          ${excludePass ? 'AND sst.pass = 0' : ''}
          AND s.station_cd = sst.station_cd
          AND s.e_status = 0
          ORDER BY sst.id
          `,
        [lineGroupId],
        async (err, results: RowDataPacket[]) => {
          if (err) {
            return reject(err);
          }
          if (!results.length) {
            return resolve([]);
          }
          return resolve(
            Promise.all(
              results.map(
                async (r): Promise<StationRaw> => ({
                  ...(r as StationRaw),
                  lines: await this.lineRepo.getByGroupId(r.station_g_cd),
                }),
              ),
            ),
          );
        },
      );
    });
  }

  async getBelongingLines(lineGroupId: number): Promise<LineRaw[]> {
    const { connection } = this.mysqlService;

    return new Promise<LineRaw[]>((resolve, reject) => {
      connection.query(
        `SELECT DISTINCT l.*
        FROM \`lines\` as l, stations as s, station_station_types as sst
        WHERE sst.line_group_cd = ?
          AND s.station_cd = sst.station_cd
          AND l.line_cd = s.line_cd
          AND s.e_status = 0`,
        [lineGroupId],
        (err, results: RowDataPacket[]) => {
          if (err) {
            return reject(err);
          }
          if (!results.length) {
            return resolve([]);
          }
          return resolve(results as LineRaw[]);
        },
      );
    });
  }

  async getAllLinesTrainTypes(
    lineGroupId: number,
  ): Promise<TrainTypeWithLineRaw[]> {
    const { connection } = this.mysqlService;

    return new Promise<TrainTypeWithLineRaw[]>((resolve, reject) => {
      connection.query(
        `SELECT DISTINCT t.*, l.*
        FROM \`lines\` as l, \`types\` as t, stations as s, station_station_types as sst
        WHERE sst.line_group_cd = ?
          AND s.station_cd = sst.station_cd
          AND sst.type_cd = t.type_cd
          AND s.e_status = 0
          AND l.line_cd = s.line_cd`,
        [lineGroupId],
        (err, results: RowDataPacket[]) => {
          if (err) {
            return reject(err);
          }
          if (!results.length) {
            return resolve([]);
          }
          return resolve(results as TrainTypeWithLineRaw[]);
        },
      );
    });
  }
}
