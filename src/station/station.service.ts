import { Injectable } from '@nestjs/common';
import { Station } from 'src/models/station.model';
import { TrainTypeRepository } from 'src/trainType/trainType.repository';
import { convertStation } from 'src/utils/convert';
import { LineRepository } from '../line/line.repository';
import { StationRepository } from './station.repository';

@Injectable()
export class StationService {
  constructor(
    private readonly stationRepo: StationRepository,
    private readonly lineRepo: LineRepository,
    private readonly trainTypeRepo: TrainTypeRepository,
  ) {}

  async findOne(id: number): Promise<Station> {
    const station = await this.stationRepo.findOneById(id);

    return convertStation(
      station,
      [await this.lineRepo.findOneCompany(station.line_cd)],
      await this.stationRepo.getTrainTypesByIds([id])[0],
    );
  }

  async findStationByGroupId(groupId: number): Promise<Station> {
    const station = await this.stationRepo.findOneById(groupId);
    const companies = await this.lineRepo.getCompaniesByLineIds([
      station.line_cd,
    ]);
    const trainTypes = await this.stationRepo.getTrainTypesByIds([
      station.station_cd,
    ]);

    return convertStation(station, companies, trainTypes[0]);
  }

  async getByCoords(
    latitude: number,
    longitude: number,
    limit?: number,
  ): Promise<Station[]> {
    const stations = await this.stationRepo.getByCoords(
      latitude,
      longitude,
      limit,
    );
    const lineIds = stations.map((s) => s.lines.map((l) => l.line_cd)).flat();
    const stationIds = stations.map((s) => s.station_cd);
    const companies = await this.lineRepo.getCompaniesByLineIds(lineIds);
    const trainTypes = await this.stationRepo.getTrainTypesByIds(stationIds);

    return Promise.all(
      stations.map((s, i) => convertStation(s, companies, trainTypes[i])),
    );
  }

  async getByLineId(lineId: number): Promise<Station[]> {
    const stations = await this.stationRepo.getByLineIds([lineId]);
    if (!stations.length) {
      return [];
    }
    const stationIds = stations.map((s) => s.station_cd);
    const stationLineIds = stations
      .map((s) => s.lines.map((l) => l.line_cd))
      .flat();
    const companies = await this.lineRepo.getCompaniesByLineIds(stationLineIds);
    const trainTypes = await this.stationRepo.getTrainTypesByIds(stationIds);
    return stations.map((s, i) => convertStation(s, companies, trainTypes[i]));
  }

  async getByName(name: string): Promise<Station[]> {
    const stations = await this.stationRepo.getByName(name);
    return stations.map((s) => convertStation(s, [], []));
  }

  async getRandomStation(): Promise<Station> {
    const station = await this.stationRepo.getRandomly();
    const companies = await this.lineRepo.getCompaniesByLineIds(
      station.lines.map((l) => l.line_cd),
    );
    return convertStation(station, companies, []);
  }
}
