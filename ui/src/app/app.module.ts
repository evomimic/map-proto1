import { NgModule, APP_INITIALIZER } from '@angular/core';
import { BrowserModule } from '@angular/platform-browser';
import { HolochainService } from './services/holochain.service';

import { AppComponent } from './app.component';
import { FooterComponent } from './footer/footer.component';
import { ViewerComponent } from './viewer/viewer.component';
import { ToolbarComponent } from './toolbar/toolbar.component';

export function initializeConnection(holochainService: HolochainService) {
  return (): Promise<any> => { 
    return holochainService.init();
  }
}

@NgModule({
  declarations: [
    AppComponent,
    FooterComponent,
    ViewerComponent,
    ToolbarComponent
  ],
  imports: [
    BrowserModule,
  ],
  providers: [HolochainService,
    { provide: APP_INITIALIZER, useFactory: initializeConnection, deps: [HolochainService], multi: true}
  ],
  bootstrap: [AppComponent]
})
export class AppModule { }
