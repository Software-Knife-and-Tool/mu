;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; fixnums
;;;
(crux:intern prelude "1+"
   (:lambda (n)
     (:if (prelude:fixnump n)
          (crux:sum n 1)
          (prelude:raise n 'prelude:1+ "not a fixnum"))))

(crux:intern prelude "1-"
   (:lambda (n)
     (:if (prelude:fixnump n)
          (crux:difference n 1)
          (prelude:raise n 'prelude:1- "not a fixnum"))))

(crux:intern prelude "truncate"
   (:lambda (n m)
     (:if (prelude:fixnump n)
          (:if (prelude:fixnump m)
               `(,(crux:quotient n m) ,@(crux:difference n (crux:product m (crux:quotient n m))))
               (prelude:raise m 'prelude:truncate "not a fixnum"))
          (prelude:raise n 'prelude:truncate "not a fixnum"))))

;;; round to negative infinity
(crux:intern prelude "floor"
   (:lambda (n m)
     (:if (prelude:fixnump n)
          (:if (prelude:fixnump m)
               `(,(crux:quotient n m) ,@(crux:difference n (crux:product m (crux:quotient n m))))
               (prelude:raise m 'prelude:rem "not a fixnum"))
          (prelude:raise n 'prelude:floor "not a fixnum"))))

;;; round to positive infinity
(crux:intern prelude "ceiling"
   (:lambda (n m)
     (:if (prelude:fixnump n)
          (:if (prelude:fixnump m)
               `(,(crux:quotient n m) ,@(crux:difference n (crux:product m (crux:quotient n m))))
               (prelude:raise m 'prelude:ceiling "not a fixnum"))
          (prelude:raise n 'prelude:ceiling "not a fixnum"))))

(crux:intern prelude "mod"
   (:lambda (n m)
     (:if (prelude:fixnump n)
          (:if (prelude:fixnump m)
               (crux:cdr (prelude:floor m n))
               (prelude:raise m 'prelude:mod "not a fixnum"))
          (prelude:raise n 'prelude:mod "not a fixnum"))))

(crux:intern prelude "rem"
   (:lambda (n m)
     (:if (prelude:fixnump n)
          (:if (prelude:fixnump m)
               (crux:cdr (prelude:truncate m n))
               (prelude:raise m 'prelude:rem "not a fixnum"))
          (prelude:raise n 'prelude:rem "not a fixnum"))))

;;; round to nearest
(crux:intern prelude "round"
   (:lambda (n m)
     (:if (prelude:fixnump n)
          (:if (prelude:fixnump m)
               (crux:difference (crux:quotient n m) (crux:difference n (crux:product m (crux:quotient n m))))
               (prelude:raise m 'prelude:round "not a fixnum"))
          (prelude:raise n 'prelude:round "not a fixnum"))))
