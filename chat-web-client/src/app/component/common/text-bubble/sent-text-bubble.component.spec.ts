import { ComponentFixture, TestBed } from '@angular/core/testing';

import { SentTextBubbleComponent } from './sent-text-bubble.component';

describe('SentTextBubbleComponent', () => {
  let component: SentTextBubbleComponent;
  let fixture: ComponentFixture<SentTextBubbleComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [SentTextBubbleComponent]
    })
    .compileComponents();
    
    fixture = TestBed.createComponent(SentTextBubbleComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
