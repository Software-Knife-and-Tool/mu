;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; fixnums
;;;
(lib:intern prelude "1+"
   (:lambda (n)
     (:if (prelude:fixnump n)
          (lib:sum n 1)
          (prelude:raise n 'prelude:1+ "not a fixnum"))))

(lib:intern prelude "1-"
   (:lambda (n)
     (:if (prelude:fixnump n)
          (lib:difference n 1)
          (prelude:raise n 'prelude:1- "not a fixnum"))))

(lib:intern prelude "truncate"
   (:lambda (n m)
     (:if (prelude:fixnump n)
          (:if (prelude:fixnump m)
               `(,(lib:quotient n m) ,@(lib:difference n (lib:product m (lib:quotient n m))))
               (prelude:raise m 'prelude:truncate "not a fixnum"))
          (prelude:raise n 'prelude:truncate "not a fixnum"))))

;;; round to negative infinity
(lib:intern prelude "floor"
   (:lambda (n m)
     (:if (prelude:fixnump n)
          (:if (prelude:fixnump m)
               `(,(lib:quotient n m) ,@(lib:difference n (lib:product m (lib:quotient n m))))
               (prelude:raise m 'prelude:rem "not a fixnum"))
          (prelude:raise n 'prelude:floor "not a fixnum"))))

;;; round to positive infinity
(lib:intern prelude "ceiling"
   (:lambda (n m)
     (:if (prelude:fixnump n)
          (:if (prelude:fixnump m)
               `(,(lib:quotient n m) ,@(lib:difference n (lib:product m (lib:quotient n m))))
               (prelude:raise m 'prelude:ceiling "not a fixnum"))
          (prelude:raise n 'prelude:ceiling "not a fixnum"))))

(lib:intern prelude "mod"
   (:lambda (n m)
     (:if (prelude:fixnump n)
          (:if (prelude:fixnump m)
               (lib:cdr (prelude:floor m n))
               (prelude:raise m 'prelude:mod "not a fixnum"))
          (prelude:raise n 'prelude:mod "not a fixnum"))))

(lib:intern prelude "rem"
   (:lambda (n m)
     (:if (prelude:fixnump n)
          (:if (prelude:fixnump m)
               (lib:cdr (prelude:truncate m n))
               (prelude:raise m 'prelude:rem "not a fixnum"))
          (prelude:raise n 'prelude:rem "not a fixnum"))))

;;; round to nearest
(lib:intern prelude "round"
   (:lambda (n m)
     (:if (prelude:fixnump n)
          (:if (prelude:fixnump m)
               (lib:difference (lib:quotient n m) (lib:difference n (lib:product m (lib:quotient n m))))
               (prelude:raise m 'prelude:round "not a fixnum"))
          (prelude:raise n 'prelude:round "not a fixnum"))))
