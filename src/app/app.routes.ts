import { Routes } from '@angular/router';
import { DeviceTableComponent } from './device-table/device-table.component';
import { HomeComponent } from './home/home.component';

export const routes: Routes = [
  { path: '', component: HomeComponent },
  { path: 'device-table', component: DeviceTableComponent }
];
