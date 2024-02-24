import { ComponentFixture, TestBed } from '@angular/core/testing';

import { ReceivedTextBubbleComponent } from './received-text-bubble.component';

describe('ChatBubbleComponent', () => {
  let component: ReceivedTextBubbleComponent;
  let fixture: ComponentFixture<ReceivedTextBubbleComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [ReceivedTextBubbleComponent]
    })
    .compileComponents();
    
    fixture = TestBed.createComponent(ReceivedTextBubbleComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
