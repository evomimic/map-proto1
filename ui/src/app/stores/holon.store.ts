import { Injectable, OnDestroy } from '@angular/core';
import { ComponentStore } from '@ngrx/component-store';
import { Observable, Subscription} from 'rxjs';
import { tap, withLatestFrom } from 'rxjs/operators';
import { HolochainService } from '../services/holochain.service';
import { HolonReceptor } from '../receptors/holon.receptor';
import { HolonEntry } from '../models/holon';

export interface HolonState {
  holons: HolonEntry[];
}

@Injectable()  //Store is a provider instance for the Container component hierarchy

export class HolonStore extends ComponentStore<HolonState> implements OnDestroy {
  private _subs = new Subscription();
  //private receptor!:HolonReceptor
  id:number = 0

  constructor(private receptor:HolonReceptor){//hcs:HolochainService){
    super({holons: []});
    //this.receptor = new HolonReceptor(hcs,"holon") 
    this.id = Math.random()
    /*this._subs.add(
      this.receptor.signalReceived$.subscribe({
        next: (holonEntry) => {
          console.log('Holon received:',holonEntry);
          this.setHolons(holonEntry!);
        },
        error: (error) => {
          console.error('An error happened while updating Holon:', error);
        },
      }) 
    )*/
  }


  /* selectors */

  selectHolons(){
    return this.select(({ holons }) => holons);
  }

  /* updaters */

  readonly addHolon = this.updater((state, holon: HolonEntry) => ({
    holons: [...state.holons, holon],
  }));

  readonly updateHolon = this.updater((state, holon: HolonEntry) => ({
    holons: [ ...state.holons.filter((entry)=>{
        return entry.id !== holon.id //? undefined : entry
      }), holon]
  }));

  readonly loadHolons = this.updater((state, holons: HolonEntry[] | null) => ({
    ...state,
    holons: holons || [],
  }));


    // effects (handles and serializes signals from the holochain network)
  /*readonly setHolons = this.effect((holon$: Observable<HolonEntry>) =>
    holon$.pipe(
      withLatestFrom(this.selectHolons()),
      tap<[HolonEntry, HolonEntry[]]>(([holon, holons]) => {
        const id = holon.id;
        const index = holons.findIndex((cur) => {
          //console.log('compare', cur, id, cur.invitation_entry_hash === id);
          return cur.id === id;
        });
        if (index > -1) {
          const modifiedHolons = [...holons];
          modifiedHolons[index] = holon;

          this.loadHolons(modifiedHolons);
        } else {
          console.debug("adding holon:",holon)
          this.updateHolon(holon)
        }
      })
    )
  );*/

  override ngOnDestroy() {  //does the parent destroy get called???
    this._subs.unsubscribe();
  }

  /* call zome functions */


  //TODO this should be an Effect 
  async loadHolonEntries():Promise<void> {
    const holons = await this.receptor.getAllHolons()
    console.debug("holons:",holons)
    this.loadHolons(holons)
  }

  getNetStatus():string {
    return this.receptor.getNetworkStatus()
  }
}
