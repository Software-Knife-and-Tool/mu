;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; fixnums
;;;
(core:intern prelude "1+"
   (:lambda (n)
     (:if (prelude:fixnump n)
          (core:sum n 1)
          (prelude:raise n 'prelude:1+ "not a fixnum"))))

(core:intern prelude "1-"
   (:lambda (n)
     (:if (prelude:fixnump n)
          (core:difference n 1)
          (prelude:raise n 'prelude:1- "not a fixnum"))))

(core:intern prelude "truncate"
   (:lambda (n m)
     (:if (prelude:fixnump n)
          (:if (prelude:fixnump m)
               `(,(core:quotient n m) ,@(core:difference n (core:product m (core:quotient n m))))
               (prelude:raise m 'prelude:truncate "not a fixnum"))
          (prelude:raise n 'prelude:truncate "not a fixnum"))))

;;; round to negative infinity
(core:intern prelude "floor"
   (:lambda (n m)
     (:if (prelude:fixnump n)
          (:if (prelude:fixnump m)
               `(,(core:quotient n m) ,@(core:difference n (core:product m (core:quotient n m))))
               (prelude:raise m 'prelude:rem "not a fixnum"))
          (prelude:raise n 'prelude:floor "not a fixnum"))))

;;; round to positive infinity
(core:intern prelude "ceiling"
   (:lambda (n m)
     (:if (prelude:fixnump n)
          (:if (prelude:fixnump m)
               `(,(core:quotient n m) ,@(core:difference n (core:product m (core:quotient n m))))
               (prelude:raise m 'prelude:ceiling "not a fixnum"))
          (prelude:raise n 'prelude:ceiling "not a fixnum"))))

(core:intern prelude "mod"
   (:lambda (n m)
     (:if (prelude:fixnump n)
          (:if (prelude:fixnump m)
               (core:cdr (prelude:floor m n))
               (prelude:raise m 'prelude:mod "not a fixnum"))
          (prelude:raise n 'prelude:mod "not a fixnum"))))

(core:intern prelude "rem"
   (:lambda (n m)
     (:if (prelude:fixnump n)
          (:if (prelude:fixnump m)
               (core:cdr (prelude:truncate m n))
               (prelude:raise m 'prelude:rem "not a fixnum"))
          (prelude:raise n 'prelude:rem "not a fixnum"))))

;;; round to nearest
(core:intern prelude "round"
   (:lambda (n m)
     (:if (prelude:fixnump n)
          (:if (prelude:fixnump m)
               (core:difference (core:quotient n m) (core:difference n (core:product m (core:quotient n m))))
               (prelude:raise m 'prelude:round "not a fixnum"))
          (prelude:raise n 'prelude:round "not a fixnum"))))
