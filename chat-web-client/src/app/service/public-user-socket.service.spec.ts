import { TestBed } from '@angular/core/testing';

import { PublicUserSocketService } from './public-user-socket.service';

describe('PublicLoginService', () => {
  let service: PublicUserSocketService;

  beforeEach(() => {
    TestBed.configureTestingModule({});
    service = TestBed.inject(PublicUserSocketService);
  });

  it('should be created', () => {
    expect(service).toBeTruthy();
  });
});
