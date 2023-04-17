import { NgModule, APP_INITIALIZER } from '@angular/core';
import { BrowserModule } from '@angular/platform-browser';
import { FormsModule, ReactiveFormsModule } from '@angular/forms';
import { HolochainService } from './services/holochain.service';

import { AppComponent } from './app.component';
import { FooterComponent } from './components/footer/footer.component';
import { ViewerComponent } from './components/viewer/viewer.component';
import { ToolbarComponent } from './components/toolbar/toolbar.component';
import { HolonReceptor } from './receptors/holon.receptor';

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
    FormsModule,
    ReactiveFormsModule
  ],
  providers: [HolochainService, // do we need this here? repition?
    { provide: APP_INITIALIZER, useFactory: initializeConnection, deps: [HolochainService], multi: true}
  ],
  bootstrap: [AppComponent]
})
export class AppModule { }
