import { Injectable, OnDestroy } from '@angular/core';
import { ComponentStore } from '@ngrx/component-store';
import { Observable, Subscription} from 'rxjs';
import { map, tap, withLatestFrom } from 'rxjs/operators';
import { HolonDescriptor } from '../models/typedescriptor';
import { TypeDescriptorReceptor } from '../receptors/typedescriptor.receptor';

export interface TypeDescriptorState {
  typedescriptors: HolonDescriptor[];
}

@Injectable()  //Store is a provider instance for the Container component hierarchy
export class TypeDescriptorStore extends ComponentStore<TypeDescriptorState> implements OnDestroy {
  private _subs = new Subscription();
  id:number = 0

  constructor(private receptor:TypeDescriptorReceptor){
    super({typedescriptors: []});
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

  selectTypeDescriptors(){
    return this.select(({ typedescriptors }) => typedescriptors);
  }

  selectTypeHeaders(){
    return this.select(({ typedescriptors }) => typedescriptors).pipe(map((types: HolonDescriptor[]) => types.map(t => {
      return t.typeheader!;
    })))
  }

  /* updaters */

  readonly addTypeDescriptor = this.updater((state, typedescriptor: HolonDescriptor) => ({
    typedescriptors: [...state.typedescriptors, typedescriptor],
  }));

  readonly updateTypeDescriptors = this.updater((state, typedescriptor: HolonDescriptor) => ({
    typedescriptors: [ ...state.typedescriptors.filter((entry)=>{
        return entry.typeheader !== typedescriptor.typeheader //? undefined : entry
      }), typedescriptor]
  }));

  readonly loadTypeDescriptors = this.updater((state, typedescriptors: HolonDescriptor[] | null) => ({
    ...state,
    typedescriptors: typedescriptors || [],
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
// all network calls are async and can have side effects 

  //TODO this should be an Effect as its an asych network call (cast the promise to an observable)
  async getAllTypeDescriptors():Promise<void> {
    const typedescriptors = await this.receptor.getAllTypeDescriptors()
    console.debug("typedescriptors:",typedescriptors)
    this.loadTypeDescriptors(typedescriptors)
  }
}
